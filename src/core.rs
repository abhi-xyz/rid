use std::env::current_dir;
use std::error::Error;
use std::path::{Path, PathBuf};
use std::{fs, path};

use crate::revert::write_log;
// use crate::revert::write_log;
use crate::utils::{current_time, trash_dir};

#[derive(Debug)]
struct Trash<'a> {
    // id: i32,
    file: &'a Path,
}

impl<'a> Trash<'a> {
    fn trash_name(&self) -> String {
        let trash_file = trash_dir().join(self.file);
        let formatted_time = current_time().format("%Y-%m-%d_%H:%M:%S").to_string();
        let trash_file = trash_file.try_exists().expect("Cant check whether trash dir exists or not");
        if !trash_file {
            self.file.file_name().unwrap().to_str().unwrap().to_string()
        } else {
            let file_name = self.file.file_name().unwrap();
           // println!("This is file name ?:{:?}", file_name);
            let stem_name = path::Path::new(file_name).file_stem().expect("failed to get file name").to_str().unwrap().to_string();
            if let Some(ext) = self.file.extension() {
                let ext = ext.to_str().unwrap().to_string();
                let n = stem_name + "." + &formatted_time + "." + &ext;
                let trash_file = self.file.with_file_name(n);
                let p = trash_file.as_path();
                self.file.with_file_name(p).to_str().unwrap().to_string()
            } else {
                let n = stem_name + "." + &formatted_time;
                self.file.with_file_name(n).to_str().unwrap().to_string()
            }
        }
    }
}

pub fn remove_file(files: Vec<PathBuf>, verbose: bool) -> Result<(), Box<dyn Error>> {
    for file in files {
        if file.is_dir() {
            eprintln!("{} is a directory.\nTry: rid -r", file.display());
            return Ok(());
        }
        let path = path::Path::new(&file).exists();
        if path {
            let trash = Trash {
                file: file.as_path(),
            };
            let trash_name = trash_dir().join(trash.trash_name());
            if verbose {
                println!("Trashed {} to {}", &file.display(), &trash_name.display());
            }
            //
            //
            let uid = current_time().format("%Y%m%d%H%M%S").to_string();
            let file_01 = current_dir().unwrap().join(&file).to_str().unwrap().to_string();
            write_log(uid.clone(), file_01.to_owned(),trash_name.to_str().unwrap().to_string(), uid).unwrap();
            //
            fs::rename(file, trash_name).unwrap();
        } else {
            eprintln!(
                "rid: cannot remove '{}': no such file or directory",
                &file.display()
            );
            return Ok(());
        }
    }
    Ok(())
}

pub fn recursive_remove(dirs: Vec<PathBuf>, verbose: bool) -> Result<(), Box<dyn Error>> {
    for dir in dirs {
        let path = path::Path::new(&dir).exists();
        if path {
            let trash = Trash {
                file: dir.as_path(),
            };
            let trash_name = trash_dir().join(trash.trash_name());
            if verbose {
                println!("Trashed {} to {}", &dir.display(), &trash_name.display());
            }
            //
            //
            let uid = current_time().format("%Y%m%d%H%M%S").to_string();
            let file_01 = current_dir().unwrap().join(&dir).to_str().unwrap().to_string();
            write_log(uid.clone(), file_01.to_owned(),trash_name.to_str().unwrap().to_string(), uid).unwrap();
            //
            fs::rename(dir, trash_name).unwrap();
        } else {
            println!(
                "rid: cannot remove '{}': no such file or directory",
                &dir.display()
            );
            return Ok(());
        }
    }
    Ok(())
}

//  TEST:

