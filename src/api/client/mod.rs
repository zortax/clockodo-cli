use std::collections::HashMap;

use reqwest::{blocking::Client, header::HeaderMap};
use serde::de::DeserializeOwned;
use serde_json::Value;

mod stopclock;

const CLIENT_NAME: &'static str = "clockodo-cli";
const CLIENT_EMAIL: &'static str = "clockodo-cli@zrtx.de";
const API_URL: &'static str = "https://my.clockodo.com";

pub trait Resource: DeserializeOwned {
  const PATH_NAME: &'static str;
  const SINGLE_NAME: &'static str;
  const LIST_NAME: &'static str;

  fn name(&self) -> &str;
}

pub trait ResourceFilter {
  const FILTER_NAME: &'static str;
  type FilterType: ToString;
}

pub struct ApiClient {
  client: Client,
}

impl<'a> ApiClient {
  pub fn new(api_user: &'a str, api_key: &'a str) -> Self {
    let mut headers = HeaderMap::new();
    headers.insert(
      "X-Clockodo-External-Application",
      format!("{};{}", CLIENT_NAME, CLIENT_EMAIL).parse().unwrap(),
    );
    headers.insert("X-ClockodoApiUser", api_user.parse().unwrap());
    headers.insert("X-ClockodoApiKey", api_key.parse().unwrap());

    let client = Client::builder()
      .user_agent(format!("{} ({})", CLIENT_NAME, CLIENT_EMAIL))
      .default_headers(headers)
      .build()
      .unwrap();

    ApiClient { client }
  }

  pub fn get(
    &self,
    url: &str,
  ) -> Result<reqwest::blocking::Response, reqwest::Error> {
    self.client.get(format!("{API_URL}{url}")).send()
  }

  pub fn get_resource<R: Resource>(
    &self,
    id: u32,
  ) -> Result<R, reqwest::Error> {
    let result = self
      .get(&format!("{}/{}", R::PATH_NAME, id))?
      .json::<HashMap<String, Value>>()?;
    Ok(serde_json::from_value(result[R::SINGLE_NAME].clone()).unwrap())
  }

  pub fn list_resources<R: Resource>(&self) -> Result<Vec<R>, reqwest::Error> {
    let result = self.get(R::PATH_NAME)?.json::<HashMap<String, Value>>()?;
    Ok(serde_json::from_value(result[R::LIST_NAME].clone()).unwrap())
  }

  pub fn list_resources_filtered<R: Resource, F: ResourceFilter>(
    &self,
    filter: F::FilterType,
  ) -> Result<Vec<R>, reqwest::Error> {
    let result = self
      .get(&format!(
        "{}/?{}={}",
        R::PATH_NAME,
        F::FILTER_NAME,
        filter.to_string()
      ))?
      .json::<HashMap<String, Value>>()?;
    Ok(serde_json::from_value(result[R::LIST_NAME].clone()).unwrap())
  }

  pub fn find_resource_by_name<R: Resource>(
    &self,
    name: &str,
  ) -> Result<Option<R>, reqwest::Error> {
    let resources = self.list_resources::<R>()?;
    Ok(resources.into_iter().find(|r| r.name() == name))
  }
}
