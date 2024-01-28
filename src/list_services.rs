use crate::{
  api::{client::ApiClient, Service},
  config::Config,
  CliError,
};

pub fn list_services(json: bool) -> Result<(), CliError> {
  let config = Config::read()?;
  let api_client = ApiClient::new(&config.api_user, &config.api_key);

  let services = api_client.list_resources::<Service>()?;

  if json {
    println!("{}", serde_json::to_string(&services).unwrap());
  } else {
    for service in services {
      print!("{}", service.name);
      if !service.active {
        print!(" {}", "(inactive)");
      }
      if let Some(note) = service.note {
        print!(" \t\tNote: {note}");
      }
      println!("");
    }
  }

  Ok(())
}
