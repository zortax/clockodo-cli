use reqwest::Url;
use serde::{Deserialize, Serialize};

use super::ApiClient;
use crate::api::TimeEntry;

#[derive(Debug, Serialize, Deserialize)]
pub struct StatusResponse {
  pub running: Option<TimeEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StopResponse {
  pub stopped: Option<TimeEntry>,
  pub running: Option<TimeEntry>,
}

impl ApiClient {
  pub fn get_status(&self) -> Result<StatusResponse, reqwest::Error> {
    Ok(self.get("/api/v2/clock")?.json::<StatusResponse>()?)
  }

  pub fn start_clock(
    &self,
    customers_id: u32,
    services_id: u32,
    billable: Option<bool>,
    projects_id: Option<u32>,
    text: Option<&str>,
  ) -> Result<StatusResponse, reqwest::Error> {
    let mut params = vec![
      ("customers_id", customers_id.to_string()),
      ("services_id", services_id.to_string()),
    ];
    if let Some(billable) = billable {
      params.push(("billable", billable.to_string()));
    }
    if let Some(projects_id) = projects_id {
      params.push(("projects_id", projects_id.to_string()));
    }
    if let Some(text) = text {
      params.push(("text", text.to_string()));
    }

    let url = Url::parse_with_params(
      &format!("{}{}", super::API_URL, "/api/v2/clock"),
      &params,
    )
    .unwrap();

    Ok(self.client.post(url).send()?.json::<StatusResponse>()?)
  }

  pub fn stop_clock(&self) -> Result<Option<StopResponse>, reqwest::Error> {
    let status = self.get_status()?;
    let entry = match status.running {
      Some(entry) => entry,
      None => return Ok(None),
    };

    Ok(Some(
      self
        .client
        .delete(format!(
          "{}{}{}",
          super::API_URL,
          "/api/v2/clock/",
          entry.id
        ))
        .send()?
        .json::<StopResponse>()?,
    ))
  }
}
