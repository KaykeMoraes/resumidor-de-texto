use std::fs::File;
use std::io::prelude::*;


pub fn read_file(path: &String) -> std::io::Result<()> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    
    Ok(())  
}
