use clap::{Parser, Subcommand};
use log::trace;
use rid::core::{recursive_remove, remove_file};
use rid::garbage_collection::gc;
use rid::history::write_history;
use std::fs::remove_dir_all;
use std::path::{Path, PathBuf};
use rid::revert::read_json_history;

#[derive(Parser)]
#[command(
    version,
    name = "rid",
    author = "Abhinandh S <ugabhi@proton.me>",
    about = "rid",
    long_about = "By default, rid does not remove directories.Use the --recursive (-r) option to remove each listed directory, too, along with all of its contents.\n
        To remove a file whose name starts with a '-', for example '-foo',\n
        use one of these commands:\n
        rid -- -foo\n
        rid ./-foo\n
        If you use rid to remove a file, it might be possible to recover the file/directory.\n
        Files are trashed to XDG specified trash directory.\n
        Example:\n
        `$HOME`/.local/share/Trash/files\n"
)]
struct Cli {
    /// Remove files
    file: Option<Vec<PathBuf>>,
    /// Remove directories and their contents recursively
    #[arg(short, long, value_name = "FILE")]
    recursive: Option<Vec<PathBuf>>,

    /// Remove directories and their contents recursively
    #[arg(short, long, value_name = "FILE")]
    force: Option<Vec<PathBuf>>,

    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,

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
    Revert {
    //    num: i8,
    }
}

fn main() {
    env_logger::init();
    let cli = Cli::parse();

    // Initialize logging based on verbosity flag
    if cli.verbose {
        trace!("verbose enabled");
    } else {
        trace!("verbose disabled");
    }

    if let Some(file) = cli.file {
        trace!("{:#?}", &file);
        remove_file(file, cli.verbose).unwrap();
    }

    if let Some(forece_file) = cli.force {
        for i in forece_file {
            if Path::new(&i).exists() {
                remove_dir_all(i).unwrap();
            } else {
                println!("Path didnt exists");
            }
        }
    }
    
    if let Some(path) = cli.recursive {
        recursive_remove(path, cli.verbose).unwrap();
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
        Some(Commands::Revert {  }) => {
            read_json_history().unwrap();
        },// Some(Commands::Revert {  }) => read_history(), // write_log(num).unwrap(),
        None => {}
    }
}
