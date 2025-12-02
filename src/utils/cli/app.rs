use std::io::{Write, stdin, stdout};
use clear_screen::clear;
use colored::{Colorize};

use crate::utils::{ai::send_prompt, cli::{path::get_path, wait::wait}, file_handle::{pdf::read_pdf, txt::read_txt}};

pub async fn run() {
    loop {
        clear();
        println!("{}", "-----------------RESUMIDOR DE TEXTOS ---------------------".blue());
        println!("--> Digite 1 se deseja resumir um arquivo em pdf");
        println!("--> Digite 2 se deseja resumir um arquivo em txt");
        println!("--> Digite 3 se deseja enviar um prompt personalizado");
        println!("--> Digite 0 para finalizar a aplicação");
        
        let mut option = String::new();
        
        print!("Opção: ");
        let _ = stdout().flush();
        stdin().read_line(&mut option).expect("Erro ao ler opção");
        
        let option: u8 = match option.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                eprintln!("Opção inválida");
                continue;
            }
        };
        
        match option {
            0 => {
                println!("{}", "Saindo...".red());
                break;
            }
            1 => {
                let path = get_path();
                
                let response = read_pdf(&path);
                
                match response {
                    Ok(text) => {
                        println!("\nResposta:\n{}", text)
                    }
                    Err(err) => {
                        eprintln!("Erro: {}", err)
                    }
                }
                wait();
            }
            2 => {
                let path = get_path();
                
                let response = read_txt(&path);
                
                match response {
                    Ok(text) => {
                        println!("\nResposta:\n{}", text)
                    }
                    Err(err) => {
                        eprintln!("Erro: {}", err)
                    }
                }
                wait();
            }
            3 => {
                print!("Digite seu prompt: ");
                stdout().flush().unwrap();
                
                let mut prompt = String::new();
                stdin().read_line(&mut prompt).expect("Erro ao ler prompt!");
                
                let response = send_prompt(&prompt).await;
                
                match response {
                    Ok(text) => {
                        println!("\nResposta:\n{}", text)
                    }
                    Err(err) => {
                        eprintln!("{}", err)
                    }
                }
                wait();
            }
            _ => {
                eprintln!("{}", "Selecione uma opção válida");
                wait(); 
            }
        }
    }
}