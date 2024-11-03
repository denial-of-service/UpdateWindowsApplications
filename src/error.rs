use std::error::Error;
use std::fmt;
#[derive(Debug)]
pub(crate) enum CustomError {
    InvalidFileError(String),
    InvalidConfigError(String),
    ResetWingetPinsError(String),
    AddWingetPinError(String),
    InvalidWingetPackageIdError(String),
    UpgradeWingetPackagesError(String),
    InvalidDesktopDirectoryError(String),
}

impl Error for CustomError {}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match self {
            CustomError::InvalidFileError(msg) => msg,
            CustomError::InvalidConfigError(msg) => msg,
            CustomError::ResetWingetPinsError(msg) => msg,
            CustomError::AddWingetPinError(msg) => msg,
            CustomError::InvalidWingetPackageIdError(msg) => msg,
            CustomError::UpgradeWingetPackagesError(msg) => msg,
            CustomError::InvalidDesktopDirectoryError(msg) => msg,
        };
        write!(f, "{msg}")
    }
}
