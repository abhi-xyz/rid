use std::error::Error;
use std::path::{Path, PathBuf};
use std::{fs, path};

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
        println!("{:?}", trash_file.display());
        let trash_file = trash_file.try_exists().expect("Err");
        if !trash_file {
            println!("trash_file_name: {}", self.file.file_name().unwrap().to_str().unwrap());
            self.file.file_name().unwrap().to_str().unwrap().to_string()
        } else {
            let file_name = self.file.file_name().unwrap();
           // println!("This is file name ?:{:?}", file_name);
            let stem_name = path::Path::new(file_name).file_stem().expect("failed to get file name").to_str().unwrap().to_string();
            if let Some(ext) = self.file.extension() {
                println!("Got {:?}", &ext);
                let ext = ext.to_str().unwrap().to_string();
                let n = stem_name + "." + &formatted_time + "." + &ext;
                let trash_file = self.file.with_file_name(n);
                println!("New trash_name: {}", &trash_file.display());
                let p = trash_file.as_path();
                self.file.with_file_name(p).to_str().unwrap().to_string()
            } else {
                println!("No etention");
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
#[cfg(test)]
mod tests {
    use std::{fs, path};
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
        remove_file(files_for_glob_test.clone(), true).unwrap();
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
        remove_file(single_files.clone(), true).unwrap();
        for i in &single_files {
            assert!(!fs::exists(i).expect("Can't check existence of file tmp_file.txt"));
        }
    }
    #[test]
    fn single_hidden_file_test() {
        let v0 = PathBuf::from(".tmp_hidden_file_for_single_file_text01.txt");
        let v1 = PathBuf::from(".tmp_hidden_file_for_single_file_text02.txt");
        let v2 = PathBuf::from(".tmp_hidden_file_for_single_file_text03.txt");
        let v3 = PathBuf::from(".tmp_hidden_file_for_single_file_text04.txt");
        let single_files = vec![v0, v1, v2, v3];
        for i in &single_files {
            fs::write(i, "some contents for the files").expect("Cant create files");
            assert!(fs::exists(i).expect("Can't check existence of file tmp_file.txt"));
        }
        remove_file(single_files.clone(), true).unwrap();
        for i in &single_files {
            assert!(!fs::exists(i).expect("Can't check existence of file tmp_file.txt"));
        }
    }
    #[test]
    fn remove_file_from_dir_test() {
        let s = Path::new("some_other").exists();
        if s {
            remove_dir_all("some_other").unwrap();
        }
        fs::create_dir_all("some_other/dir").unwrap();
        fs::write("some_other/dir/test.txt", "some contents for testing").unwrap();
        let v3 = PathBuf::from("some_other/dir/test.txt");
        let single_files = vec![v3];
        remove_file(single_files, true).expect("Err with my function");
        assert!(!fs::exists("some_other/dir/test.txt").expect("Can't check existence of file some/dir/for/testing/tmp_file.txt"));
        recursive_remove(vec![path::Path::new("some_other").to_path_buf()], true).expect("Err with my function");
        assert!(!fs::exists("some_other").expect("Can't check existence of file some/dir/for/testing/tmp_file.txt"));
    }
    #[test]
    fn remove_file_from_hidden_dir_test() {
        let s = Path::new(".some_hidden").exists();
        if s {
            remove_dir_all(".some_hidden").unwrap();
        }
        fs::create_dir_all(".some_hidden/dir").unwrap();
        fs::write(".some_hidden/dir/test.txt", "some contents for testing").unwrap();
        let v3 = PathBuf::from(".some_hidden/dir/test.txt");
        let single_files = vec![v3];
        remove_file(single_files, true).expect("Err with my function");
        assert!(!fs::exists(".some_hidden/dir/test.txt").expect("Can't check existence of file some/dir/for/testing/tmp_file.txt"));
        recursive_remove(vec![path::Path::new(".some_hidden").to_path_buf()], true).expect("Err with my function");
        assert!(!fs::exists(".some_hidden").expect("Can't check existence of file some/dir/for/testing/tmp_file.txt"));
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
        recursive_remove(test_dir, true).expect("Err with my function");
        assert!(!fs::exists("some")
            .expect("Can't check existence of file some/dir/for/testing/tmp_file.txt"));
        assert!(!fs::exists("some/dir/for")
            .expect("Can't check existence of file some/dir/for/testing/tmp_file.txt"));
        assert!(!fs::exists("some/dir/for/testing")
            .expect("Can't check existence of file some/dir/for/testing/tmp_file.txt"));
    }
    #[test]
    fn recursive_hidden_dir_remove_test() {
        let s = Path::new(".some").exists();
        if s {
            remove_dir_all(".some").unwrap();
        }
        fs::create_dir_all(".some/dir/for/testing").unwrap();
        let test_dir = vec![PathBuf::from(".some")];
        fs::write(".some/test.txt", "some contents for testing").unwrap();
        recursive_remove(test_dir, true).expect("Err with my function");
        assert!(!fs::exists(".some")
            .expect("Can't check existence of file some/dir/for/testing/tmp_file.txt"));
        assert!(!fs::exists(".some/dir/for")
            .expect("Can't check existence of file some/dir/for/testing/tmp_file.txt"));
        assert!(!fs::exists(".some/dir/for/testing")
            .expect("Can't check existence of file some/dir/for/testing/tmp_file.txt"));
    }
}
