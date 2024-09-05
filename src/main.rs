use std::{env, error::Error, fs::File, ffi::OsString, process}; 

mod csvreader;

fn main() {
    let arecord = csvreader::csvcon().unwrap();
    println!("{}",arecord[1].p2);
    //println!("first p1: {}",arecord[0].p1);

}
