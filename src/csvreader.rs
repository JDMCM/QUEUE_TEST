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
    pub fn new(i1: f64, i2: f64, p1: &Particle, p2: &Particle, event_time: f64) -> Self {
        Rec {
            p1: i1,
            p2: i2,
            time: event_time,
            p1x: p1.p.x(),
            p1y: p1.p.y(),
            p1z: p1.p.z(),
            p1vx: p1.v.x(),
            p1vy: p1.v.y(),
            p1vz: p1.v.z(),
            p1r: p1.r,
            p2x: p2.p.x(),
            p2y: p2.p.y(),
            p2z: p2.p.z(),
            p2vx: p2.v.x(),
            p2vy: p2.v.y(),
            p2vz: p2.v.z(),
            p2r: p2.r,
        }
    }

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