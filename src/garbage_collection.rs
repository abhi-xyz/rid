use chrono::{DateTime, Utc};
use std::error::Error;
use std::time::{Duration, UNIX_EPOCH};
use std::vec;

#[allow(dead_code)]
#[derive(Debug)]
struct Trash {
    file: String,
    date: i64,
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

pub fn gc(_date: &i8) -> Result<(), Box<dyn Error>> {
    //let tmp_trash_files = [
    //    path::Path::new("/home/abhi/.local/share/Trash/files/fd.lua"),
    //    path::Path::new("/home/abhi/.local/share/Trash/files/fd"),
    //    path::Path::new("/home/abhi/.local/share/Trash/files/n.org"),
    //    path::Path::new("/home/abhi/.local/share/Trash/files/test_001.org"),
    //    path::Path::new("/home/abhi/.local/share/Trash/files/test_002.org"),
    //];
    //for i in tmp_trash_files {
    //    let n = i.metadata()?;
    //    let file_name = i.file_name().unwrap();
    //    let f= file_name.to_str().unwrap().to_string();
    //    println!("trash meta:\nfilename: {}\nacessed time: {}.{}\n", f, n.st_atime(), n.st_atime_nsec());
    //}
    //let mut trash = vec![
    //    Trash { file: String::from("file1.txt"), date: 1690123456 },
    //    Trash { file: String::from("file2.txt"), date: 1698123456 },
    //];
    //println!("Got {} for gc", date);
    //let attr = fs::metadata("/home/abhi/.local/share/Trash/files/fd.lua")?;
    //println!("Metadata of fd.lua:\n{:#?}", &attr);
    //println!("ACCESSED: {:?}", attr.accessed().unwrap());
    //// Example timestamps
    //let modified_tv_sec = 1730443746;
    //let modified_tv_nsec = 874530307;
    //
    //let accessed_tv_sec = 1731302640;
    //let accessed_tv_nsec = 797661558;
    //
    //let created_tv_sec = 1730443746;
    //let created_tv_nsec = 874530307;
    //
    //println!("Modified: {}", convert_system_time(modified_tv_sec, modified_tv_nsec));
    //println!("Accessed: {}", convert_system_time(accessed_tv_sec, accessed_tv_nsec));
    //println!("Created: {}", convert_system_time(created_tv_sec, created_tv_nsec));
    //let a = convert_system_time(modified_tv_sec, modified_tv_nsec);
    //println!("{}", a);
    //
    let _today: u64 = 20241101064906;

    get_items_for_gc(Some(30));

    sort_by_date();
    Ok(())
}

fn get_items_for_gc(period: Option<u8>) -> Vec<u64> {
    let b = period.is_none();
    if b {
        println!("no time period specifed:\nconsidering 30 as default\ncleaning all files and dirs in trash for longer than 30 days");
    } else {
        println!("considering 30 as default\ncleaning all files and dirs in trash for longer than 30 days");
    }

    let mut access_time_vec: Vec<u64> = vec![20241101064906, 20241101064907, 20241101064905];
    access_time_vec.sort();
    println!("sorted from vec: {:#?}", access_time_vec);
    access_time_vec
}

fn sort_by_date() {
    let mut access_time: [u64; 3] = [20241101064906, 20241101064907, 20241101064905];
    access_time.sort();
    println!("sorted from slice: {:#?}", access_time);
}

fn convert_system_time(tv_sec: u64, tv_nsec: u32) -> String {
    let duration = Duration::new(tv_sec, tv_nsec);
    let _datetime = UNIX_EPOCH + duration;
    let naive_datetime = DateTime::from_timestamp(tv_sec as i64, tv_nsec).unwrap();
    let datetime: DateTime<Utc> =
        DateTime::from_naive_utc_and_offset(naive_datetime.naive_utc(), Utc);
    datetime.format("%Y-%m-%d %H:%M:%S.%f").to_string()
}

#[allow(dead_code)]
pub fn dry_run() {}
