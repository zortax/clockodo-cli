use std::collections::HashMap;

use crate::{
  api::{client::ApiClient, Customer, CustomersFilter, Project},
  config::Config,
  Args, CliError,
};

pub fn list_projects(args: &Args) -> Result<(), CliError> {
  let config = Config::read()?;
  let api_client = ApiClient::new(&config.api_user, &config.api_key);

  let mut projects: HashMap<String, Vec<String>> = HashMap::new();
  let customers = api_client.list_resources::<Customer>()?;
  for customer in customers {
    let customer_projects = api_client
      .list_resources_filtered::<Project, CustomersFilter>(customer.id)?;
    projects.insert(
      customer.name,
      customer_projects
        .into_iter()
        .map(|project| project.name)
        .collect(),
    );
  }

  if args.json {
    println!("{}", serde_json::to_string(&projects).unwrap());
  } else {
    for (customer, projects) in projects {
      println!("{}: ", customer);
      if projects.is_empty() {
        println!("  [no projects]");
      }
      for project in projects {
        println!("  {}", project);
      }
    }
  }

  Ok(())
}
