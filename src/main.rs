use std::path::PathBuf;

use clap::{Parser, Subcommand};
use gr::{recursive_remove, remove_file};

#[derive(Parser)]
#[command(version, about, long_about = "By default, gr does not remove directories. Use the --recursive (-r) option to remove each listed directory, too, along with all of its contents.")]
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
        println!("Value for name: {}", file.display());
        remove_file(file).unwrap();
    }

    if let Some(path) = cli.recursive.as_deref() {
        println!("Value for config: {}", path.display());
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
    use std::fs;
    use std::path::Path;

    use super::*;

    #[test]
    fn single_file_test() {
        fs::write("tmp_file.txt", "test contents").unwrap();
        assert!(fs::exists("tmp_file.txt").is_ok());
        let p = Path::new("tmp_file.txt");
        remove_file(p).unwrap();
        assert!(!fs::exists("tmp_file.txt").expect("Can't check existence of file tmp_file.txt"));
    }

    #[test]
    fn recursive_remove_test() {
        fs::create_dir_all("some/dir/for/testing").unwrap();
        fs::write("some/dir/for/testing/tmp_file.txt", "test contents").unwrap();
        let p = Path::new("some/dir/for/testing/tmp_file.txt");
        assert!(fs::exists("some/dir/for/testing/tmp_file.txt").is_ok());
        recursive_remove(p).unwrap();
        assert!(!fs::exists("some/dir/for/testing/tmp_file.txt").expect("Can't check existence of file some/dir/for/testing/tmp_file.txt"));
    }
}
