use std::io::{Write, stdin, stdout};

pub fn wait() {
    print!("\nPressione enter para continuar ");
    stdout().flush().unwrap();
    
    let mut _enter = String::new();
    stdin().read_line(&mut _enter).unwrap();
}