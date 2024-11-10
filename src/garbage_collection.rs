use std::error::Error;

#[derive(Debug)]
struct Trash {
    file: String,
    date: i64
}

pub trait Metadata {
    fn get_file_name(&self) -> &str;
    fn get_deleted_date(&self) -> i64;
}

impl Metadata for Trash {
    fn get_file_name(&self) -> &str {
        &self.file
    }
    fn get_deleted_date(&self) -> i64 {
        self.date
    }
}

pub fn gc(date: &i8) -> Result<(), Box<dyn Error>> {
    let mut trash = vec![
        Trash { file: String::from("file1.txt"), date: 1690123456 },
        Trash { file: String::from("file2.txt"), date: 1698123456 },
    ];
    println!("Got {} for gc", date);
    Ok(())
}

#[allow(dead_code)]
pub fn dry_run() {
    
}
