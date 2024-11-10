#[cfg(test)]
mod tests {
    use std::fs::remove_dir_all;
    use std::fs;
    use std::path::{Path, PathBuf};
    use rid::{recursive_remove, remove_file};

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
            PathBuf::from("glob_test_file_008.txt")
        ];
        for i in &files_for_glob_test {
            println!("{}", i.display());
            fs::write(i, "some contents for the files").expect("Cant create files");
            assert!(fs::exists(i).expect("Can't check existence of file glob_test_file_00x.txt"));
        }
        remove_file(files_for_glob_test.clone()).unwrap();
        for i in files_for_glob_test { assert!(!fs::exists(i).expect("Can't check existence of file tmp_file.txt")); }
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
        for i in &single_files { assert!(!fs::exists(i).expect("Can't check existence of file tmp_file.txt")); }
    }

    #[test]
    fn recursive_remove_test() {
        let s =  Path::new("some").exists();
        if s { remove_dir_all("some").unwrap(); }
        fs::create_dir_all("some/dir/for/testing").unwrap();
        let dirs = PathBuf::from("some/dir/for/testing");
        let test_dir = vec![dirs];
        fs::write("some/test.txt", "some contents for testing").unwrap();
        recursive_remove(test_dir).unwrap();
        assert!(!fs::exists("some").expect("Can't check existence of file some/dir/for/testing/tmp_file.txt"));
        assert!(!fs::exists("some/dir/for").expect("Can't check existence of file some/dir/for/testing/tmp_file.txt"));
        assert!(!fs::exists("some/dir/for/testing").expect("Can't check existence of file some/dir/for/testing/tmp_file.txt"));
    }
}
