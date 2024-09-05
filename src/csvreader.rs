use std::{env, error::Error, fs::File, ffi::OsString, process}; 
use csv;
use serde;

#[derive(Debug, serde::Deserialize)]
//#[serde(rename_all = "PascalCase")]
pub struct Rec {
    pub I1:f64,
    pub p1:f64, 
    pub p2:f64,
    pub time:f64,
    pub p1x:f64,
    pub p1y:f64,
    pub p1z:f64,
    pub p1vx:f64,
    pub p1vy:f64,
    pub p1vz:f64,
    pub p1r:f64,
    pub p2x:f64, 
    pub p2y:f64, 
    pub p2z:f64, 
    pub p2vx:f64, 
    pub p2vy:f64, 
    pub p2vz:f64, 
    pub p2r:f64
}

pub fn csvcon() ->Result<Vec<Rec>, Box<dyn Error>>  {
    //let mut file_path: String = String::new();
    //println!("Enter CSV file path :");
    //std::io::stdin().read_line(&mut file_path);
    //file_path = file_path.replace("\"","");
    let mut rdr = csv::Reader::from_path("C:/Users/JD/Documents/GitHub/PYTHON-SCRIPTS/Hello_friend.csv")?;

    let mut matrix:Vec<Rec> = Vec::new();
    for result in rdr.deserialize() {
        let record:Rec = result?;
        matrix.push(record);
    }
    return Ok(matrix);
   
}