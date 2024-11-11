use std::path::PathBuf;

use chrono::{DateTime, Local};
use dirs::data_local_dir;
use log::trace;

pub fn trash_dir() -> PathBuf {
    let trash_dir = data_local_dir()
        .expect("Failed to get local data directory")
        .join("Trash/files");
    trace!("trash_dir: {}", &trash_dir.display());
    trash_dir
}

pub fn current_time() -> DateTime<Local> {
    let c_time = Local::now();
    trace!("{}", &c_time);
    c_time
}
