use chrono::Local;
use serde::Serialize;
use serde_json::json;

const DATE_FORMAT: &str = "%Y-%m-%d %H:%M:%S";

use crate::{
  api::{
    client::ApiClient, Billable, Customer, EntryType, Project, Service,
    TimeEntry,
  },
  config::Config,
  Args, CliError,
};

#[derive(Debug, Serialize)]
pub struct JsonOutput {
  pub running: bool,
  pub time_entry: Option<TimeEntryOutput>,
}

#[derive(Debug, Serialize)]
pub struct TimeEntryOutput {
  id: u32,
  customer: String,
  project: Option<String>,
  entry_type: EntryType,
  service: Option<String>,
  billable: Billable,
  description: String,
  time_insert: String,
  time_since: String,
  duration: String,
}

impl TimeEntryOutput {
  pub fn from_time_entry(
    entry: TimeEntry,
    customer: &str,
    service: Option<&str>,
    project: Option<&str>,
  ) -> Result<Self, CliError> {
    Ok(TimeEntryOutput {
      id: entry.id,
      customer: customer.into(),
      project: project.map(|p| p.into()),
      entry_type: entry.entry_type,
      service: service.map(|s| s.into()),
      billable: entry.billable,
      description: entry.text.unwrap_or("[empty]".to_string()),
      time_insert: entry
        .time_insert
        .parse::<chrono::DateTime<chrono::Utc>>()?
        .with_timezone(&Local)
        .format(DATE_FORMAT)
        .to_string(),
      time_since: entry
        .time_since
        .parse::<chrono::DateTime<chrono::Utc>>()?
        .with_timezone(&Local)
        .format(DATE_FORMAT)
        .to_string(),
      duration: {
        let time = entry.time_since.parse::<chrono::DateTime<chrono::Utc>>()?;
        let duration =
          chrono::Utc::now().signed_duration_since(time).num_seconds();
        format!(
          "{:0>2}:{:0>2}:{:0>2}",
          duration / 3600,
          duration % 3600 / 60,
          duration % 60
        )
      },
    })
  }

  pub fn print(&self) {
    println!("ID:\t\t{}", self.id);
    println!("Customer:\t{}", self.customer);
    if let Some(project) = &self.project {
      println!("Project:\t{}", project);
    }
    println!("Entry type:\t{}", self.entry_type);
    if let Some(service) = &self.service {
      println!("Service:\t{}", service);
    }
    println!("Billable:\t{}", self.billable);
    println!("Description:\t{}", self.description);
    println!("Time inserted:\t{}", self.time_insert);
    println!("Time started:\t{}", self.time_since);
    println!("Duration:\t{}", self.duration);
  }
}

pub fn status(args: &Args, duration: bool) -> Result<(), CliError> {
  let config = Config::read()?;
  let api_client = ApiClient::new(&config.api_user, &config.api_key);

  let status = api_client.get_status()?;

  let output = JsonOutput {
    running: status.running.is_some(),
    time_entry: match status.running {
      None => None,
      Some(time_entry) => {
        let customer =
          api_client.get_resource::<Customer>(time_entry.customers_id)?;
        let service = match time_entry.services_id {
          Some(services_id) => {
            Some(api_client.get_resource::<Service>(services_id)?.name)
          }
          None => None,
        };
        let project = match time_entry.projects_id {
          Some(projects_id) => {
            Some(api_client.get_resource::<Project>(projects_id)?.name)
          }
          None => None,
        };

        Some(TimeEntryOutput::from_time_entry(
          time_entry,
          &customer.name,
          service.as_deref(),
          project.as_deref(),
        )?)
      }
    },
  };

  if args.json {
    if duration {
      if let Some(time_entry) = &output.time_entry {
        println!(
          "{}",
          json!({
              "duration": time_entry.duration,
          })
          .to_string()
        );
      } else {
        println!(
          "{}",
          json!({
          "running": false,
          "duration": null,
          })
          .to_string()
        );
      }
    } else {
      println!("{}", serde_json::to_string(&output).unwrap());
    }
  } else {
    if let Some(time_entry) = output.time_entry {
      if duration {
        println!("{}", time_entry.duration);
        return Ok(());
      } else {
        println!("Stopclock is running\n");
        time_entry.print();
      }
    } else {
      println!("No time entry running.");
    }
  }

  Ok(())
}
