use crate::error::CustomError;
use dirs::desktop_dir;
use std::fs;
use std::path::PathBuf;

pub(crate) fn remove_desktop_shortcuts() -> Result<(), CustomError> {
    // Get the path to the desktop directory
    let desktop_path: PathBuf = match desktop_dir() {
        Some(dir) => dir,
        None => return Err(CustomError::InvalidDesktopDirectoryError("Failed to locate desktop directory".to_string()))
    };

    // Iterate over all files in the desktop directory
    let read_dir = match fs::read_dir(desktop_path) {
        Ok(dir) => dir,
        Err(_) => return Err(CustomError::InvalidDesktopDirectoryError("Failed to open desktop directory".to_string()))
    };
    for dir_entry in read_dir {
        if let Ok(entry) = dir_entry {
            let path: PathBuf = entry.path();
            // Check if the file is a shortcut (.lnk)
            if let Some(ext) = path.extension() {
                if ext == "lnk" {
                    // Delete the file
                    if let Err(error) = fs::remove_file(&path) {
                        eprintln!("Failed to delete {}: {error}", path.display());
                    }
                }
            }
        }
    }
    Ok(())
}
