use std::{ cmp, error::Error}; 
use csv;
use serde;
use std::collections::VecDeque;

use cmp::Ordering;

use crate::{particle::Particle, vectors::Vector};

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

impl Rec {
    pub fn p1(&self) -> Particle {
        Particle::new(
            Vector::new(self.p1x, self.p1y, self.p1z),
            Vector::new(self.p1vx, self.p1vy, self.p1vz),
            self.p1r,
            1.0
        )
    }

    pub fn p2(&self) -> Particle {
        Particle::new(
            Vector::new(self.p2x, self.p2y, self.p2z),
            Vector::new(self.p2vx, self.p2vy, self.p2vz),
            self.p2r,
            1.0
        )
    }
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

pub fn csvcon(file: &String) ->Result<VecDeque<Rec>, Box<dyn Error>>  {
    let file_path = file.replace("\"","").replace("\\","/");
    println!("{}1",file_path);
    let mut rdr = csv::Reader::from_path(file_path.trim())?;

    let mut matrix:VecDeque<Rec> = VecDeque::new();
    for result in rdr.deserialize() {
        let record:Rec = result?;
        matrix.push_back(record);
    }
    return Ok(matrix);
   
}