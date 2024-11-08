use std::error::Error;
use std::path;
use std::path::Path;

use dirs::data_local_dir;

pub fn remove_file(file: &Path) -> Result<(),Box<dyn Error>> {
    let p = path::Path::new(file).exists();
    let dest = if p {
        let trash_dir = data_local_dir().expect("Failed to get local data directory").join("Trash/files").join(file);
        println!("{}", trash_dir.display());
        "ds".to_string()
    } else {
        return Ok(());
    };
    println!("{}", dest);
    Ok(())
}

pub fn recursive_remove(dir: &Path) -> Result<(),Box<dyn Error>> {
    println!("{}", &dir.display());
    Ok(())
}
