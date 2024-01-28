use serde::Serialize;
use thiserror::Error;

use crate::{
  api::{client::ApiClient, Customer, Project, Service},
  config::Config,
  status::TimeEntryOutput,
  CliError,
};

#[derive(Debug, Serialize)]
struct StopOutput {
  stopped: bool,
  time_entry: Option<TimeEntryOutput>,
}

#[derive(Debug, Error)]
pub enum StopError {
  #[error("The stopclock is not currently running")]
  NotRunning,

  #[error(
    "Couldn't stop the clock (the running time entry doesn't seem to be \
     stopped, even though the API request was successfull)"
  )]
  NotStopped,
}

pub fn stop(json: bool) -> Result<(), CliError> {
  let config = Config::read()?;
  let api_client = ApiClient::new(&config.api_user, &config.api_key);

  let entry = api_client
    .stop_clock()?
    .ok_or(StopError::NotRunning)?
    .stopped
    .ok_or(StopError::NotStopped)?;

  let customer = api_client
    .get_resource::<Customer>(entry.customers_id)?
    .name;

  let project = match entry.projects_id {
    Some(id) => Some(api_client.get_resource::<Project>(id)?.name),
    None => None,
  };

  let service = match entry.services_id {
    Some(id) => Some(api_client.get_resource::<Service>(id)?.name),
    None => None,
  };

  let entry = TimeEntryOutput::from_time_entry(
    entry,
    &customer,
    service.as_deref(),
    project.as_deref(),
  )?;

  if json {
    println!(
      "{}",
      serde_json::to_string(&StopOutput {
        stopped: true,
        time_entry: Some(entry),
      })
      .unwrap()
    );
  } else {
    println!("{}", "The stopclock has been stopped.\n");
    entry.print();
  }

  Ok(())
}
