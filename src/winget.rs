use crate::error::CustomError;
use std::process::{Command, Stdio};

pub fn reset_winget_package_pins() -> Result<(), CustomError> {
    let result = Command::new("powershell")
        .arg("-Command")
        .args(&["winget", "pin", "reset", "--force"])
        .stdout(Stdio::null())
        .status();
    match result {
        Ok(exit_status) if exit_status.success() => Ok(()),
        _ => Err(CustomError::ResetWingetPinsError("Command to reset all winget pins failed".to_string())),
    }
}

pub fn add_winget_package_pin(package_id: &str) -> Result<(), CustomError> {
    let result = Command::new("powershell")
        .arg("-Command")
        .args(&["winget", "pin", "add", package_id, "--blocking", "--force"])
        .stdout(Stdio::null())
        .status();
    match result {
        Ok(exit_status) if exit_status.success() => Ok(()),
        Ok(_) => Err(CustomError::InvalidWingetPackageIdError(format!("'No package with the id {package_id}' installed on this system."))),
        Err(_) => Err(CustomError::AddWingetPinError(format!("Failed to add winget pin for package id '{package_id}'"))),
    }
}

pub fn upgrade_winget_packages() -> Result<(), CustomError> {
    let result = Command::new("powershell")
        .arg("-Command")
        .args(&["winget", "upgrade", "--all", "--silent", "--disable-interactivity"])
        .status();
    match result {
        Ok(exit_status) if exit_status.success() => Ok(()),
        _ => Err(CustomError::UpgradeWingetPackagesError("Command to upgrade all winget packages failed".to_string())),
    }
}
