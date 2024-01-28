use core::fmt;

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use self::client::{Resource, ResourceFilter};

pub mod client;

#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum Billable {
  NotBillable = 0,
  Billable = 1,
  AlreadyBilled = 2,
}

impl fmt::Display for Billable {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Billable::NotBillable => write!(f, "not billable"),
      Billable::Billable => write!(f, "billable"),
      Billable::AlreadyBilled => write!(f, "already billed"),
    }
  }
}

#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum EntryType {
  TimeEntry = 1,
  LumpSumValue = 2,
  LumpSumService = 3,
}

impl fmt::Display for EntryType {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      EntryType::TimeEntry => write!(f, "time entry"),
      EntryType::LumpSumValue => write!(f, "lump sum value"),
      EntryType::LumpSumService => write!(f, "lump sum service"),
    }
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimeEntry {
  pub id: u32,
  pub customers_id: u32,
  pub projects_id: Option<u32>,
  pub users_id: u32,
  pub time_insert: String,
  pub billable: Billable,
  pub time_since: String,
  pub text: Option<String>,

  #[serde(rename = "type")]
  pub entry_type: EntryType,

  pub services_id: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Customer {
  pub id: u32,
  pub name: String,
}

impl Resource for Customer {
  const PATH_NAME: &'static str = "/api/v2/customers";
  const SINGLE_NAME: &'static str = "customer";
  const LIST_NAME: &'static str = "customers";

  fn name(&self) -> &str {
    &self.name
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
  pub id: u32,
  pub name: String,
  pub customers_id: u32,
}

impl Resource for Project {
  const PATH_NAME: &'static str = "/api/v2/projects";
  const SINGLE_NAME: &'static str = "project";
  const LIST_NAME: &'static str = "projects";

  fn name(&self) -> &str {
    &self.name
  }
}

pub struct CustomersFilter;

impl ResourceFilter for CustomersFilter {
  const FILTER_NAME: &'static str = "filter[customers_id]";
  type FilterType = u32;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Service {
  pub id: u32,
  pub name: String,
  pub active: bool,
  pub note: Option<String>,
}

impl Resource for Service {
  const PATH_NAME: &'static str = "/api/v2/services";
  const SINGLE_NAME: &'static str = "service";
  const LIST_NAME: &'static str = "services";

  fn name(&self) -> &str {
    &self.name
  }
}
