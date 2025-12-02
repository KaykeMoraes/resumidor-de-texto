use std::io::{Write, stdin, stdout};

pub fn get_path() -> String {
    print!("Cole o diretório do seu arquivo aqui: ");
    stdout().flush().unwrap();
    
    let mut path = String::new();
    stdin().read_line(&mut path).unwrap();
    
    path.trim().to_string()
}