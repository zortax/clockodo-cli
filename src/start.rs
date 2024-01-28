use thiserror::Error;

use crate::{
  api::{client::ApiClient, Customer, Project, Service},
  config::Config,
  status::{JsonOutput, TimeEntryOutput},
  CliError,
};

#[derive(Debug, Error)]
pub enum StartError {
  #[error("Could not find customer {0}")]
  CustomerNotFound(String),

  #[error("Could not find service {0}")]
  ServiceNotFound(String),

  #[error("Could not find project {0}")]
  ProjectNotFound(String),

  #[error(
    "Failed to start the stopclock (API request was successfull, but no time \
     entry seems to be present)"
  )]
  FailedToStart,
}

pub fn start(
  json: bool,
  customer: &str,
  service: &str,
  project: Option<&str>,
  billable: Option<bool>,
  description: Option<&str>,
) -> Result<(), CliError> {
  let config = Config::read()?;
  let api_client = ApiClient::new(&config.api_user, &config.api_key);

  let customer = api_client
    .find_resource_by_name::<Customer>(customer)?
    .ok_or(StartError::CustomerNotFound(customer.into()))?;

  let service = api_client
    .find_resource_by_name::<Service>(service)?
    .ok_or(StartError::ServiceNotFound(service.into()))?;

  let project = if let Some(project) = project {
    Some(
      api_client
        .find_resource_by_name::<Project>(project)?
        .ok_or(StartError::ProjectNotFound(project.into()))?,
    )
  } else {
    None
  };

  let response = api_client.start_clock(
    customer.id,
    service.id,
    billable,
    project.as_ref().map(|p| p.id),
    description,
  )?;

  let entry = response.running.ok_or(StartError::FailedToStart)?;
  let output = TimeEntryOutput::from_time_entry(
    entry,
    &customer.name,
    Some(&service.name),
    project.map(|p| p.name.clone()).as_deref(),
  )?;

  if json {
    println!(
      "{}",
      serde_json::to_string(&JsonOutput {
        running: true,
        time_entry: Some(output),
      })
      .unwrap()
    );
  } else {
    println!("{}", "Started the stopclock.\n");
    output.print();
  }

  Ok(())
}
