mod command;
mod config;
mod error;
mod progress;
mod winget;

use crate::config::Config;
use crate::error::CustomError;
use owo_colors::OwoColorize;
use progress::Progress;
use std::thread::sleep;
use std::time::Duration;

const ERROR_MESSAGE_DISPLAY_DURATION: Duration = Duration::from_secs(30);
const SUCCESS_MESSAGE_DISPLAY_DURATION: Duration = Duration::from_secs(10);
const MINIMUM_NUMBER_OF_TASKS: u8 = 2;

fn main() {
    if let Err(error) = run_script() {
        eprintln!("Error: {}", error.red());
        sleep(ERROR_MESSAGE_DISPLAY_DURATION);
    }
}

fn run_script() -> Result<(), CustomError> {
    let config = Config::new()?;
    let mut total_number_of_tasks = MINIMUM_NUMBER_OF_TASKS;
    if !config.package_ids_not_to_update.is_empty() {
        total_number_of_tasks += 1;
    };
    if config.delete_desktop_shortcut_files {
        total_number_of_tasks += 1
    };
    let mut progress = Progress::new(total_number_of_tasks);

    progress.update_progress("Resetting winget package pins...");
    winget::reset_winget_package_pins()?;

    if !config.package_ids_not_to_update.is_empty() {
        progress.update_progress("Adding winget package pins...");
        for blacklisted_package_id in config.package_ids_not_to_update.iter() {
            match winget::add_winget_package_pin(blacklisted_package_id) {
                Ok(_) => {}
                Err(CustomError::InvalidWingetPackageIdError(message)) => {
                    print_warning_message(&message)
                }
                Err(err) => return Err(err),
            }
        }
    }

    progress.update_progress("Upgrading winget packages, this may take a while...");
    winget::upgrade_winget_packages()?;

    if config.delete_desktop_shortcut_files {
        progress.update_progress("Removing desktop shortcuts...");
        command::remove_desktop_shortcuts()?;
    }

    println!("Done! Ending script in {} seconds...", SUCCESS_MESSAGE_DISPLAY_DURATION.as_secs());
    sleep(SUCCESS_MESSAGE_DISPLAY_DURATION);
    Ok(())
}

fn print_warning_message(message: &str) {
    eprintln!("Warning: {}", message.yellow());
}
