use std::fs;
use std::io::{Write, stdout};

pub fn get_user_file_path() -> String {
    print!("Digite apenas o nome do arquivo dentro da pasta:");
    let _ = stdout().flush();
    
    let mut filename = String::new();
    std::io::stdin().read_line(&mut filename).unwrap();
    filename = filename.trim().to_string();

    let file_path = format!("/app/input/{}", filename);

    if !fs::metadata(&file_path).is_ok() {
        println!("Arquivo não encontrado: {}", file_path);
    }

    file_path
}
