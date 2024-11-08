use std::error::Error;
use std::path::Path;

pub fn recursive_remove(dir: &Path) -> Result<(),Box<dyn Error>> {
    println!("{}", &dir.display());
    Ok(())
}
