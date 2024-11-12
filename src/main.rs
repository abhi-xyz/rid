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
    name = "rid",
    author = "Abhinandh S <ugabhi@proton.me>",
    about = "rid",
    long_about = "By default, rid does not remove directories.\nUse the --recursive (-r) option to remove each listed directory, too, along with all of its contents.\nTo remove a file whose name starts with a '-', for example '-foo',\n
    use one of these commands:\n
    rid -- -foo\n
    rid ./-foo\n
    If you use rid to remove a file, it might be possible to recover the file/directory.\nFiles are trashed to XDG specified trash directory.\n
    Example:\n
    `$HOME`/.local/share/Trash/files\n"
)]
struct Cli {
    /// Remove files
    file: Option<Vec<PathBuf>>,
    /// Remove directories and their contents recursively
    #[arg(short, long, value_name = "FILE")]
    recursive: Option<Vec<PathBuf>>,

    /// For testing porpose wont work
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
    /// alpha stage
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
