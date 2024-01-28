use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
  pub api_user: String,
  pub api_key: String,
}

#[derive(Debug, Error)]
pub enum ConfigError {
  #[error("IO error: {0}")]
  Io(#[from] std::io::Error),

  #[error("Failed to parse config: {0}")]
  TomlParse(#[from] toml::de::Error),

  #[error("Failed to serialize config: {0}")]
  TomlSerialize(#[from] toml::ser::Error),
}

impl Config {
  #[inline]
  fn config_path() -> Result<std::path::PathBuf, ConfigError> {
    let config_path = dirs::config_dir()
      .ok_or_else(|| {
        std::io::Error::new(std::io::ErrorKind::NotFound, "No config dir found")
      })?
      .join("clockodo-cli")
      .join("config.toml");

    Ok(config_path)
  }

  pub fn read() -> Result<Self, ConfigError> {
    let config_path = Self::config_path()?;
    let config = std::fs::read_to_string(config_path)?;
    let config: Config = toml::from_str(&config)?;

    Ok(config)
  }

  pub fn write(&self) -> Result<(), ConfigError> {
    let config_path = Self::config_path()?;
    let config_dir = config_path.parent().unwrap();
    std::fs::create_dir_all(config_dir)?;

    let config = toml::to_string_pretty(self)?;
    std::fs::write(config_path, config)?;

    Ok(())
  }
}
