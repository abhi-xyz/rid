use std::error::Error;
use std::fs::{self, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::vec;

use crate::utils::{current_time, LogId};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct TrashHistory {
    pub original_path: String,
    pub trash_path: String,
    pub deleted_at: String,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct TrashMeta {
    pub unique_id: String,
    pub history: TrashHistory
}

pub fn write_log(
    unique_id: String,
    original_path: String,
    trash_path: String,
    deleted_at: String,
) -> Result<(), Box<dyn Error>>  {
    let mut file = OpenOptions::new()
        .create(true)  // Create the file if it doesn't exist
        .append(true)  // Append to the file if it already exists
        .open("/home/abhi/.local/share/rid/rid_history.log")?;

    writeln!(file, "{}", unique_id)?;
    writeln!(file, "{}", original_path)?;
    writeln!(file, "{}", trash_path)?;
    writeln!(file, "{}", deleted_at)?;
    writeln!(file, "----------------------------")?;

    Ok(())
}

pub fn read_json_history() -> Result<(), Box<dyn Error>> {
    println!("hello from revert");
    let file = fs::File::open("/home/abhi/.local/share/rid/rid_history.log").unwrap();
    let reader = BufReader::new(file);

    let mut a_vec = vec::Vec::new();
    for line in reader.lines() {
       let l = line.unwrap();
        a_vec.push(l.clone());
       // println!("{}", &l);
    }
     println!("{:#?}", &a_vec);
    let len = a_vec.len();
        let se = &a_vec[len-1]; // last one
     println!("FROM VEC: {:#?}", se);
     //let v = a_vec.rsplit(pred)
        
    //
    //
    let c_time = current_time();
    let id = c_time.format("%Y%m%d%H%M%S").to_string();
    let id = id.as_str();
    println!("{}", &id);
    let log_id: LogId = id.parse().unwrap();
    println!("{:#?}", log_id);
    //
    //



    Ok(())
}
