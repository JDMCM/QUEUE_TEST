use std::{env, error::Error, fs::File, ffi::OsString, process};

pub fn csvcon() -> Result<(), Box<dyn Error>> {
    let mut file_path: String = String::new();
    println!("Enter CSV file path :");
    std::io::stdin().read_line(&mut file_path).unwrap();
    file_path = file_path.replace("\"","");
    println!("{file_path}");
    Ok(())
}