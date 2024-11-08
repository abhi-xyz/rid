use std::path::PathBuf;

use clap::{Parser, Subcommand};
use rid::{recursive_remove, remove_file};

#[derive(Parser)]
#[command(version, about, long_about = "By default, rid does not remove directories. Use the --recursive (-r) option to remove each listed directory, too, along with all of its contents.")]
struct Cli {
    /// Remove files
    file: Option<PathBuf>,
    /// Remove directories and their contents recursively
    #[arg(short, long, value_name = "FILE")]
    recursive: Option<PathBuf>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Turn debugging information on
    Debug {
        #[arg(short, long)]
        list: bool,
    },
}

fn main() {
    let cli = Cli::parse();

    if let Some(file) = cli.file.as_deref() {
        remove_file(file).unwrap();
    }

    if let Some(path) = cli.recursive.as_deref() {
        recursive_remove(path).unwrap();
    }

    match &cli.command {
        Some(Commands::Debug { list }) => {
            if *list {
                println!("Printing testing lists...");
            } else {
                println!("Not printing testing lists...");
            }
        }
        None => {}
    }
}

#[cfg(test)]
mod tests {
    use std::fs::{remove_dir_all, remove_file};
    use std::fs;
    use std::path::Path;

    use super::*;

    #[test]
    fn single_file_test() {
        let t_file: String = "temp_file.org".into(); 
        let s =  Path::new(&t_file).exists();
        if s {
            remove_file(&t_file).unwrap();
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
