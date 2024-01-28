use clap::{Parser, Subcommand};
use list_projects::list_projects;
use list_services::list_services;
use login::login;
use serde::Serialize;
use start::{start, StartError};
use status::status;
use stop::{stop, StopError};
use thiserror::Error;

mod api;
mod config;
mod list_projects;
mod list_services;
mod login;
mod start;
mod status;
mod stop;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
  #[command(subcommand)]
  command: Command,

  /// Prefer JSON output
  #[arg(long = "json", short = 'j')]
  json: bool,
}

#[derive(Subcommand, Debug)]
enum Command {
  /// Logs the user in
  Login {
    /// The API user
    #[arg(long = "api-user", short = 'u')]
    api_user: Option<String>,

    /// The API key
    #[arg(long = "api-key", short = 'k')]
    api_key: Option<String>,
  },

  /// Starts the stopclock
  Start {
    /// The customer name
    #[arg(long = "customer", short = 'c')]
    customer: String,

    /// The project name
    #[arg(long = "project", short = 'p')]
    project: Option<String>,

    /// The service name
    #[arg(long = "service", short = 's')]
    service: String,

    /// The billable flag
    #[arg(long = "billable", short = 'b')]
    billable: Option<bool>,

    /// The description
    #[arg(long = "description", short = 'd')]
    description: Option<String>,
  },

  /// Stops the stopclock
  Stop,

  /// Shows the current status of the stopclock
  Status {
    /// Show the duration of the current time entry
    #[arg(long = "duration", short = 'd')]
    duration: bool,
  },

  /// Lists available customers/projects
  ListProjects,

  /// Lists available services
  ListServices,
}

#[derive(Debug, Error)]
enum CliError {
  #[error(transparent)]
  Config(#[from] config::ConfigError),

  #[error("API request failed: {0}")]
  Api(#[from] reqwest::Error),

  #[error("Failed to parse time: {0}")]
  Time(#[from] chrono::ParseError),

  #[error(transparent)]
  Start(#[from] StartError),

  #[error(transparent)]
  Stop(#[from] StopError),
}

#[derive(Debug, Serialize)]
struct JsonError {
  error_message: String,
}

fn main() {
  let args = Args::parse();

  let result = match args.command {
    Command::Login { api_user, api_key } => login(api_user, api_key),
    Command::Status { duration } => status(&args, duration),
    Command::ListProjects => list_projects(&args),
    Command::ListServices => list_services(args.json),
    Command::Start {
      customer,
      project,
      service,
      billable,
      description,
    } => start(
      args.json,
      &customer,
      &service,
      project.as_deref(),
      billable,
      description.as_deref(),
    ),
    Command::Stop => stop(args.json),
  };

  if let Err(err) = result {
    if args.json {
      let json_error = JsonError {
        error_message: err.to_string(),
      };
      println!("{}", serde_json::to_string(&json_error).unwrap());
    } else {
      eprintln!("Error: {}", err);
    }
    std::process::exit(1);
  }
}
