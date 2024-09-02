use std::{env, error::Error, fs::File, ffi::OsString, process}; 
use csv;
use serde;

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Rec {
    p1:f64, 
    p2:f64,
    time:f64,
    p1x:f64,
    p1y:f64,
    p1z:f64,
    p1vx:f64,
    p1vy:f64,
    p1vz:f64,
    p1r:f64,
    p2x:f64, 
    p2y:f64, 
    p2z:f64, 
    p2vx:f64, 
    p2vy:f64, 
    p2vz:f64, 
    p2r:f64
}

pub fn csvcon() ->Result<Vec<Rec>, Box<dyn Error>>  {
    let mut file_path: String = String::new();
    println!("Enter CSV file path :");
    std::io::stdin().read_line(&mut file_path);
    file_path = file_path.replace("\"","");

    let file = File::open(file_path)?;
    
   

    let mut rdr = csv::Reader::from_reader(file);

    let mut record:Vec<Rec> = Vec::new();
    for result in rdr.deserialize() {
        record.push(result.unwrap());
    }
    return Ok(record);
   
}