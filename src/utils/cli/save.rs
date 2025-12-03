use std::io::{Write, stdin, stdout};

use crate::utils::{file_handle::create_file::create_md_file};

pub fn save_option(text: &String) {
    loop {
        print!("\nVocê deseja salvar esse resumo em um arquivo? [s/n] ");
        stdout().flush().unwrap();
        
        let mut answer = String::new();
        stdin().read_line(&mut answer).unwrap();
        
        if answer.trim().to_ascii_lowercase() == "s" {
            create_md_file(text).expect("Erro ao criar arquivo!");
            println!("Arquivo criado com sucesso!");
            break;
        } else if answer.trim().to_ascii_lowercase() == "n" {
            break;
        } else {
            eprintln!("Digite uma opção válida!")
        }
    }
}