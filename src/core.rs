use std::env::current_dir;
use std::error::Error;
use std::path::{Path, PathBuf};
use std::{fs, path};

use chrono::Local;
use dirs::data_local_dir;

/*
FIX: can't remove items from hidden dirs
rid .github/workflows/release.yml
thread 'main' panicked at src/main.rs:41:27:
called `Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound, message: "No such file or directory" }
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
*/
pub fn remove_file(file: Vec<PathBuf>) -> Result<(), Box<dyn Error>> {
    for i in file {
        let p = path::Path::new(&i).exists();
        if i.is_dir() {
            eprintln!("{} is a directory.\nTry: rid -r", i.display());
            return Ok(());
        }
        if p {
            let trash_dir = data_local_dir()
                .expect("Failed to get local data directory")
                .join("Trash/files");
            if trash_dir.join(&i).exists() {
                let c_time = Local::now();
                let formatted_time = c_time.format("%Y-%m-%d_%H:%M:%S").to_string();
                let stem_name = Path::new(&i).file_stem().expect("Err").to_str().unwrap();
                let ext = Path::new(&i).extension().unwrap().to_str().unwrap();
                let new_name = stem_name.to_string() + "." + &formatted_time + "." + ext;
                let new = format!("{}", trash_dir.join(new_name).display());
                // println!("pwd: {}", current_dir().unwrap().to_str().unwrap());
                // println!("target: {}", &new);
                fs::rename(i, &new)?;
            } else {
                // println!("pwd: {}", current_dir().unwrap().to_str().unwrap());
                // println!("target: {}", &trash_dir.join(&i).display());
                fs::rename(&i, trash_dir.join(&i))?;
            }
        } else {
            eprintln!(
                "rid: cannot remove '{}': No such file or directory",
                &i.display()
            );
            return Ok(());
        };
    }
    Ok(())
}
pub fn recursive_remove(dir: Vec<PathBuf>) -> Result<(), Box<dyn Error>> {
    for i in dir {
        let p = path::Path::new(&i).exists();
        if p {
            let trash_dir = data_local_dir()
                .expect("Failed to get local data directory")
                .join("Trash/files")
                .join(&i);
            if trash_dir.exists() {
                let c_time = Local::now();
                let formatted_time = c_time.format("%Y-%m-%d_%H:%M:%S").to_string();
                let file_name = Path::new(&trash_dir)
                    .file_name()
                    .expect("Failed to extract file names from Trash");
                let new_trash_path = data_local_dir()
                    .expect("Failed to get local data directory")
                    .join("Trash/files")
                    .join(file_name);
                let new_name = format!("{}.{}", &new_trash_path.display(), &formatted_time);
                eprintln!(
                    "rid: {} already exists\nTrashed as {}",
                    &trash_dir.display(),
                    &new_name
                );
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
            eprintln!(
                "rid: cannot remove '{}': No such file or directory",
                &i.display()
            );
            return Ok(());
        };
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::fs::remove_dir_all;
    use std::path::{Path, PathBuf};

    use crate::core::{recursive_remove, remove_file};

    // use core::{recursive_remove, remove_file};
    #[test]
    fn glob_test() {
        // FIX: need fix, not an good test
        let files_for_glob_test = vec![
            PathBuf::from("glob_test_file_001.txt"),
            PathBuf::from("glob_test_file_002.txt"),
            PathBuf::from("glob_test_file_003.txt"),
            PathBuf::from("glob_test_file_004.txt"),
            PathBuf::from("glob_test_file_005.txt"),
            PathBuf::from("glob_test_file_006.txt"),
            PathBuf::from("glob_test_file_007.txt"),
            PathBuf::from("glob_test_file_008.txt"),
        ];
        for i in &files_for_glob_test {
            println!("{}", i.display());
            fs::write(i, "some contents for the files").expect("Cant create files");
            assert!(fs::exists(i).expect("Can't check existence of file glob_test_file_00x.txt"));
        }
        remove_file(files_for_glob_test.clone()).unwrap();
        for i in files_for_glob_test {
            assert!(!fs::exists(i).expect("Can't check existence of file tmp_file.txt"));
        }
    }

    #[test]
    fn single_file_test() {
        let v0 = PathBuf::from("temp_file_for_single_file_text01.txt");
        let v1 = PathBuf::from("temp_file_for_single_file_text02.txt");
        let v2 = PathBuf::from("temp_file_for_single_file_text03.txt");
        let v3 = PathBuf::from("temp_file_for_single_file_text04.txt");
        let single_files = vec![v0, v1, v2, v3];
        for i in &single_files {
            fs::write(i, "some contents for the files").expect("Cant create files");
            assert!(fs::exists(i).expect("Can't check existence of file tmp_file.txt"));
        }
        remove_file(single_files.clone()).unwrap();
        for i in &single_files {
            assert!(!fs::exists(i).expect("Can't check existence of file tmp_file.txt"));
        }
    }

    #[test]
    fn recursive_remove_test() {
        let s = Path::new("some").exists();
        if s {
            remove_dir_all("some").unwrap();
        }
        fs::create_dir_all("some/dir/for/testing").unwrap();
        let test_dir = vec![PathBuf::from("some")];
        fs::write("some/test.txt", "some contents for testing").unwrap();
        recursive_remove(test_dir).expect("Err with my function");
        assert!(!fs::exists("some")
            .expect("Can't check existence of file some/dir/for/testing/tmp_file.txt"));
        assert!(!fs::exists("some/dir/for")
            .expect("Can't check existence of file some/dir/for/testing/tmp_file.txt"));
        assert!(!fs::exists("some/dir/for/testing")
            .expect("Can't check existence of file some/dir/for/testing/tmp_file.txt"));
    }
}
