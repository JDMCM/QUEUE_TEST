use std::{env, error::Error, fs::File, ffi::OsString, process}; 

mod csvreader;

fn main() {
    csvreader::csvcon();
    println!("Hello, world!");
}
