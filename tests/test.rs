#[cfg(test)]
mod tests {
    use std::fs;


    #[test]
    fn github_test() {
        fs::write("file_for_github.txt", "some contents for the files").expect("Cant create files for github");
        fs::create_dir_all("some/dir/for/testing").expect("Cant create files for github");
    }
}
