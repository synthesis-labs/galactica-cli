use std::{
    fs::{self, read_to_string},
    path::Path,
};

use galactica_lib::{auth::DiscordAccessToken, specs::HistoryEntry};
use serde::{Deserialize, Serialize};

use crate::errors::Error;

const CONFIG_PATH: &str = ".galactica/config.json";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub api_url: String,
    pub token: Option<DiscordAccessToken>,
    pub history: Vec<HistoryEntry>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            // api_url: "http://127.0.0.1:8000".to_string(),
            api_url: "https://ikmqanq88j.eu-west-1.awsapprunner.com".to_string(),
            token: None,
            history: vec![],
        }
    }
}

pub fn read() -> Result<Config, Error> {
    if exists() {
        let contents = read_to_string(Path::new(&config_file_path())).map_err(|e| {
            Error::ConfigError(
                config_file_path(),
                format!("Error while reading: {}", e.to_string()),
            )
        })?;

        let config: Config = serde_json::from_str(contents.as_str()).map_err(|e| {
            Error::ConfigError(
                CONFIG_PATH.to_string(),
                format!("Error while deserializing: {}", e.to_string()),
            )
        })?;

        Ok(config)
    } else {
        Err(Error::ConfigError(
            CONFIG_PATH.to_string(),
            "Does not exist, please run setup first!".to_string(),
        ))
    }
}

pub fn write(config: &Config) -> Result<(), Error> {
    let json = serde_json::to_string(config).map_err(|_| {
        Error::ConfigError(
            CONFIG_PATH.to_string(),
            "Unable to serialize config object".to_string(),
        )
    })?;

    if let Some(parent_dir) = Path::new(&config_file_path()).parent() {
        fs::create_dir_all(parent_dir).map_err(|e| {
            Error::ConfigError(
                config_file_path(),
                format!(
                    "Unable to create directories while attempting to write config file due to {}",
                    e.to_string()
                ),
            )
        })?;
    }

    fs::write(config_file_path(), json.as_str()).map_err(|e| {
        Error::ConfigError(
            config_file_path(),
            format!("Unable to write to config file due to {}", e.to_string()),
        )
    })?;

    Ok(())
}

pub fn exists() -> bool {
    Path::new(&config_file_path()).exists()
}

pub fn config_file_path() -> String {
    format!(
        "{}/{}",
        dirs::home_dir().unwrap().to_str().unwrap(),
        CONFIG_PATH
    )
}
