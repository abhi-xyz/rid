use clap::{Parser, Subcommand};
use log::trace;
use std::path::PathBuf;

pub mod core;
pub mod garbage_collection;
pub mod history;
pub mod utils;

use core::{recursive_remove, remove_file};
use garbage_collection::gc;
use history::write_history;

#[derive(Parser)]
#[command(
    version,
    about,
    long_about = "By default, rid does not remove directories. Use the --recursive (-r) option to remove each listed directory, too, along with all of its contents."
)]
struct Cli {
    /// Remove files
    file: Option<Vec<PathBuf>>,
    /// Remove directories and their contents recursively
    #[arg(short, long, value_name = "FILE")]
    recursive: Option<Vec<PathBuf>>,

    #[arg(short, long)]
    json: Option<bool>,

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
    Gc {
        // #[arg(short, long)]
        date: i8,
    },
}

fn main() {
    env_logger::init();
    let cli = Cli::parse();

    if let Some(file) = cli.file {
        trace!("{:#?}", &file);
        remove_file(file).unwrap();
    }

    if let Some(path) = cli.recursive {
        recursive_remove(path).unwrap();
    }
    if let Some(t) = cli.json {
        if t {
            write_history().unwrap();
        } else {
            println!("try true");
        }
    }

    match &cli.command {
        Some(Commands::Debug { list }) => {
            if *list {
                println!("Printing testing lists...");
            } else {
                println!("Not printing testing lists...");
            }
        }
        Some(Commands::Gc { date }) => gc(date).unwrap(),
        None => {}
    }
}
