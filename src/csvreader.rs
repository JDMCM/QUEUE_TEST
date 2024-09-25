use std::{ cmp, error::Error}; 
use csv;
use serde;
use std::collections::VecDeque;

use cmp::Ordering;

#[derive(Debug, serde::Deserialize,Clone, Copy)]
//#[serde(rename_all = "PascalCase")]
pub struct Rec {
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

impl PartialOrd for Rec {
    fn partial_cmp(&self, other: &Rec) -> Option<Ordering> {
        self.time.partial_cmp(&other.time)
    }
}


impl PartialEq for Rec {
    fn eq(&self, other: &Rec) -> bool {
        self.time == other.time
    }
}

pub fn csvcon() ->Result<VecDeque<Rec>, Box<dyn Error>>  {
    let mut file_path: String = String::new();
    println!("Enter CSV file path :");
    std::io::stdin().read_line(&mut file_path);
    file_path = file_path.replace("\"","").replace("\\","/");
    println!("{}1",file_path);
    let mut rdr = csv::Reader::from_path(file_path.trim())?;

    let mut matrix:VecDeque<Rec> = VecDeque::new();
    for result in rdr.deserialize() {
        let record:Rec = result?;
        matrix.push_back(record);
    }
    return Ok(matrix);
   
}