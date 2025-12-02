use std::{fs::File, io::{Error, Read}};

pub fn read_txt(path: &String) -> Result<String, Error> {
    let mut file = File::open(path).expect("Erro ao abrir arquivo de texto!");
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    
    Ok(contents)
}
