use std::io::Write;

use crate::{config::Config, CliError};

pub fn login(
  api_user: Option<String>,
  api_key: Option<String>,
) -> Result<(), CliError> {
  let api_user = api_user.unwrap_or_else(|| {
    print!("API user: ");
    std::io::stdout().flush().unwrap();
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
  });

  let api_key = api_key.unwrap_or_else(|| {
    print!("API key: ");
    std::io::stdout().flush().unwrap();
    let input = rpassword::read_password().unwrap();
    input.trim().to_string()
  });
  let config = Config { api_user, api_key };

  if let Err(err) = config.write() {
    eprintln!("Failed to write config: {}", err);
    std::process::exit(1);
  }

  println!("Logged in successfully");
  Ok(())
}
