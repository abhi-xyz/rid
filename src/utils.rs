use std::path::PathBuf;

use chrono::{DateTime, Local};
use dirs::data_local_dir;
use log::trace;

/// # Returns the path to the user's local trash directory.
///
/// The returned value depends on the operating system and is either a `Some`, containing a value from the following table, or a `None`.
///
/// |Platform | Value                                                | Example                                   |
/// | ------- | -----------------------------------------------------| ----------------------------------------- |
/// | Linux   | `$XDG_DATA_HOME` or `$HOME`/.local/share/Trash/files | /home/alice/.local/share/Trash/files      |
pub fn trash_dir() -> PathBuf {
    let trash_dir = data_local_dir()
        .expect("Failed to get local data directory")
        .join("Trash/files");
    trace!("trash_dir: {}", &trash_dir.display());
    trash_dir
}

/// Returns a `DateTime<Local>` which corresponds to the current date and time.
///
/// # Example
///
/// ```
/// let formatted_time = current_time().format("%Y-%m-%d_%H:%M:%S").to_string();
/// ```
pub fn current_time() -> DateTime<Local> {
    let c_time = Local::now();
    trace!("{}", &c_time);
    c_time
}
