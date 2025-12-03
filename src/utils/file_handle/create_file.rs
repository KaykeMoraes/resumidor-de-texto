use std::{fs::File, io::{Error, Write, stdin, stdout}, path::PathBuf};
use dirs::download_dir;

pub fn create_md_file(text: &String) -> Result<File, Error> {
    print!("\nDigite o nome do arquivo: ");
    stdout().flush().unwrap();
    
    let mut file_name = String::new();
    stdin().read_line(&mut file_name).unwrap();
    
    let dir_path: PathBuf = download_dir().expect("Diretório de downloads não encontrado!");
    let file_path = dir_path.join(format!("{}.md", file_name.trim()));
    
    let mut file = File::create_new(&file_path)?;
    file.write_all(text.as_bytes())?;
   
    println!("\nArquivo salvo na pasta Downloads.");
    
    Ok(file)
}