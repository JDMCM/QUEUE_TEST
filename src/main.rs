use std::{env, error::Error, fs::File, ffi::OsString, process}; 

mod csvreader;
mod bucketqueue;

fn main() {
    let mut arecord = csvreader::csvcon().unwrap();



    let mut data : Vec<Vec<csvreader::Rec>> = vec![Vec::new();500];

    
    let mut max:f64= 0.0;

    for i in 0..arecord.len() {
        let poppy = arecord[i];
        if(poppy.time > max) {
            max = poppy.time;
        }
    }

    while !arecord.is_empty() {
        let poppy = arecord.pop().unwrap();
        let index:usize = (poppy.time/(max/500.0)).floor() as usize;
        data[index].push(poppy);

    }

    println!("{}",data[100].len());
    //println!("first p1: {}",arecord[0].p1);

}
