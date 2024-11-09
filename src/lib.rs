use std::env::current_dir;
use std::error::Error;
use std::{fs, path};
use std::path::Path;

use chrono::Local;
use dirs::data_local_dir;

pub mod history;

pub fn remove_file(file: &Path) -> Result<(),Box<dyn Error>> {
    let p = path::Path::new(file).exists();
    if p {
        let trash_dir = data_local_dir().expect("Failed to get local data directory").join("Trash/files");
        dbg!("{}", &trash_dir);
        if trash_dir.join(file).exists() {
            let c_time = Local::now();
            let formatted_time = c_time.format("%Y-%m-%d_%H:%M:%S").to_string();
            let stem_name = Path::new(&file).file_stem().expect("Err").to_str().unwrap();
            let ext = Path::new(&file).extension().unwrap().to_str().unwrap();
            let new_name = stem_name.to_string() + "." + &formatted_time + "." + ext;
            let new = format!("{}",trash_dir.join(new_name).display());
            println!("pwd: {}", current_dir().unwrap().to_str().unwrap());
            println!("target: {}", &new);
            fs::rename(file, &new)?;
        } else {
            println!("pwd: {}", current_dir().unwrap().to_str().unwrap());
            println!("target: {}", &trash_dir.join(file).display());
            fs::rename(file, trash_dir.join(file))?;
        }
    } else {
        eprintln!("rid: cannot remove '{}': No such file or directory", &file.display());
        return Ok(());
    };
    Ok(())
}

pub fn recursive_remove(dir: &Path) -> Result<(),Box<dyn Error>> {
    let p = path::Path::new(dir).exists();
    if p {
        let trash_dir = data_local_dir().expect("Failed to get local data directory").join("Trash/files").join(dir);
        if trash_dir.exists() {
            let c_time = Local::now();
            let formatted_time = c_time.format("%Y-%m-%d_%H:%M:%S").to_string();
            eprintln!("rid: {} already exists", &trash_dir.display());
            let new_name = format!("{}.{}",&trash_dir.to_str().unwrap().to_string(), &formatted_time);
            println!("pwd: {}", current_dir().unwrap().to_str().unwrap());
            println!("target: {}", &new_name);
            fs::rename(dir, new_name)?;
        } else {
            // moves the file or dir to trash with same name. 
            println!("pwd: {}", current_dir().unwrap().to_str().unwrap());
            println!("target: {}", &trash_dir.display());
            fs::rename(dir, &trash_dir)?;
        }
        trash_dir.to_str().unwrap().to_string()
    } else {
        eprintln!("rid: cannot remove '{}': No such file or directory", &dir.display());
        return Ok(());
    };
    Ok(())
}
