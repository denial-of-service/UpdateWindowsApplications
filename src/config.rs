use crate::error::CustomError;
use serde::Deserialize;
use std::fs::read_to_string;

const CONFIG_FILE_NAME: &str = "config.toml";

#[derive(Debug, Deserialize)]
pub(crate) struct Config {
    pub(crate) delete_desktop_shortcut_files: bool,
    pub(crate) package_ids_not_to_update: Vec<String>,
}

impl Config {
    pub(crate) fn new() -> Result<Config, CustomError> {
        let file_content = match read_to_string(CONFIG_FILE_NAME) {
            Ok(content) => content,
            Err(_) => {
                return Err(CustomError::InvalidFileError(
                    format!("Config file {CONFIG_FILE_NAME} not found or isn't valid UTF-8"))
                )
            }
        };

        match toml::from_str(&file_content) {
            Ok(config) => Ok(config),
            Err(_) => Err(CustomError::InvalidConfigError(format!("Failed to parse config file {CONFIG_FILE_NAME}"))),
        }
    }
}
