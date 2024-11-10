use std::env::current_dir;
use std::error::Error;
use std::{fs, path};
use std::path::{Path, PathBuf};

use chrono::Local;
use dirs::data_local_dir;

pub mod garbage_collection;
pub mod history;

pub fn remove_file(file: Vec<PathBuf>) -> Result<(),Box<dyn Error>> {
    for i in file {
        let p = path::Path::new(&i).exists();
        if i.is_dir() {
            eprintln!("{} is a directory.\nTry: rid -r", i.display());
            return Ok(());
        }
        if p {
            let trash_dir = data_local_dir().expect("Failed to get local data directory").join("Trash/files");
            if trash_dir.join(&i).exists() {
                let c_time = Local::now();
                let formatted_time = c_time.format("%Y-%m-%d_%H:%M:%S").to_string();
                let stem_name = Path::new(&i).file_stem().expect("Err").to_str().unwrap();
                let ext = Path::new(&i).extension().unwrap().to_str().unwrap();
                let new_name = stem_name.to_string() + "." + &formatted_time + "." + ext;
                let new = format!("{}",trash_dir.join(new_name).display());
                // println!("pwd: {}", current_dir().unwrap().to_str().unwrap());
                // println!("target: {}", &new);
                fs::rename(i, &new)?;
            } else {
                // println!("pwd: {}", current_dir().unwrap().to_str().unwrap());
                // println!("target: {}", &trash_dir.join(&i).display());
                fs::rename(&i, trash_dir.join(&i))?;
            }
        } else {
            eprintln!("rid: cannot remove '{}': No such file or directory", &i.display());
            return Ok(());
        };  
    }
    Ok(())
}
pub fn recursive_remove(dir: Vec<PathBuf>) -> Result<(),Box<dyn Error>> {
    for i in dir {
    let p = path::Path::new(&i).exists();
    if p {
        let trash_dir = data_local_dir().expect("Failed to get local data directory").join("Trash/files").join(&i);
        if trash_dir.exists() {
            let c_time = Local::now();
            let formatted_time = c_time.format("%Y-%m-%d_%H:%M:%S").to_string();
            let file_name = Path::new(&trash_dir).file_name().expect("Failed to extract file names from Trash");
            let new_trash_path = data_local_dir().expect("Failed to get local data directory").join("Trash/files").join(file_name);
            let new_name = format!("{}.{}",&new_trash_path.display(), &formatted_time);
            eprintln!("rid: {} already exists\nTrashed as {}", &trash_dir.display(), &new_name);
            // println!("pwd: {}", current_dir().unwrap().to_str().unwrap());
            // println!("target: {}", &new_name);
            fs::rename(i, new_name)?;
        } else {
            // moves the file or dir to trash with same name. 
            println!("pwd: {}", current_dir().unwrap().to_str().unwrap());
            println!("target: {}", &trash_dir.display());
            fs::rename(i, &trash_dir)?;
        }
        trash_dir.to_str().unwrap().to_string()
    } else {
        eprintln!("rid: cannot remove '{}': No such file or directory", &i.display());
        return Ok(());
    };
    }
    Ok(())
}
