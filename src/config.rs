use crate::error::CustomError;
use serde::Deserialize;
use std::env;
use std::fs::read_to_string;

const CONFIG_FILE_NAME: &str = "config.toml";

#[derive(Debug, Deserialize)]
pub(crate) struct Config {
    pub(crate) delete_desktop_shortcut_files: bool,
    pub(crate) package_ids_not_to_update: Vec<String>,
}

impl Config {
    pub(crate) fn new() -> Result<Config, CustomError> {
        // Get the directory of the current executable
        let exe_path = env::current_exe().map_err(|_| {
            CustomError::InvalidFileError("Failed to get the file path to this program".to_string())
        })?;

        // Get the path of the config file by resolving it relative to the executable's directory
        let config_file_path = exe_path.parent()
            .ok_or_else(|| CustomError::InvalidFileError("Path to this program is invalid".to_string()))?
            .join(CONFIG_FILE_NAME);

        // Read the config file
        let file_content = read_to_string(config_file_path).map_err(|_| {
            CustomError::InvalidFileError(format!(
                "Config file {CONFIG_FILE_NAME} not found or isn't valid UTF-8"
            ))
        })?;

        // Parse the TOML content into the Config struct
        toml::de::from_str(&file_content).map_err(|_| {
            CustomError::InvalidConfigError(format!(
                "Failed to parse config file {CONFIG_FILE_NAME}"
            ))
        })
    }
}
