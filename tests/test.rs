#[cfg(test)]
mod tests {
    use std::fs::remove_dir_all;
    use std::fs;
    use std::path::Path;
    use rid::{recursive_remove, remove_file};
    
    #[test]
    fn single_file_test() {
        let t_file: String = "temp_file.org".into(); 
        let s =  Path::new(&t_file).exists();
        if s {
            remove_file(Path::new(&t_file)).unwrap();
        }
        assert!(!fs::exists(&t_file).expect("Can't check existence of file tmp_file.txt"));
        fs::write(&t_file, "test contents").unwrap();
        assert!(fs::exists(&t_file).is_ok());
        let p = Path::new(&t_file);
        remove_file(p).unwrap();
        assert!(!fs::exists(t_file).expect("Can't check existence of file tmp_file.txt"));
    }

    #[test]
    fn recursive_remove_test() {
        let s =  Path::new("some").exists();
        if s { remove_dir_all("some").unwrap(); }
        fs::create_dir_all("some/dir/for/testing").unwrap();
        fs::write("some/test.txt", "some contents for testing").unwrap();
        recursive_remove(Path::new("some")).unwrap();
        assert!(!fs::exists("some").expect("Can't check existence of file some/dir/for/testing/tmp_file.txt"));
        assert!(!fs::exists("some/dir/for").expect("Can't check existence of file some/dir/for/testing/tmp_file.txt"));
        assert!(!fs::exists("some/dir/for/testing").expect("Can't check existence of file some/dir/for/testing/tmp_file.txt"));
    }
}
