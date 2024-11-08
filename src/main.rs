use std::path::PathBuf;

use clap::{Parser, Subcommand};
use gr::recursive_remove;

#[derive(Parser)]
#[command(version, about, long_about = "By default, gr does not remove directories.  Use the --recursive (-r) option to remove each listed directory, too, along with all of its contents.")]
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
