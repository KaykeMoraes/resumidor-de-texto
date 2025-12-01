use std::io::{self, Write, stdin, stdout};
use colored::{Color, Colorize};
use clear_screen::clear;
use resumidor_de_pdf::modules::{ai::{AI, summarize_text}, file_handle::{path::{get_user_file_path}, pdf::read_pdf, txt::read_txt}};

pub async fn run() {
    let ai = AI::new();
    loop {
        clear();
        println!("{}", "---------------RESUMIDOR DE TEXTO---------------".color(Color::BrightCyan));
        println!("--> Digite 0 para sair\n--> Digite 1 se quiser resumir texto de um pdf\n--> Digite 2 se quiser resumir um texto de um arquivo .txt");
        print!("Opção: ");
        let _ = stdout().flush();
        
        let mut option = String::new();
        stdin().read_line(&mut option).expect("Erro ao ler a opção.");
        
        let option: u8 = option.trim().parse().unwrap();
        
        match option {
            0 => {
                println!("Saindo..."); 
                break;
            } 
            1 => {
                let path = get_user_file_path(); 
                let text = read_pdf(path.trim());
                let response;
                if text.is_ok() {
                    response = summarize_text(&text.unwrap(), &ai).await.expect("Erro ao resumir texto.");
                    println!("\nResumo:\n{}\n", response);
                } else {
                    eprintln!("\nErro ao ler pdf. Tente novamente.\n")
                }
                
                let mut enter = String::new();
                print!("Pressine Enter para continuar");
                let _ = stdout().flush();
                stdin().read_line(&mut enter).unwrap();
            }
            2 => {
                let path = get_user_file_path(); 
                let text = read_txt(path.trim());
                let response;
                if text.is_ok() {
                    response = summarize_text(&text.unwrap(), &ai).await.expect("Erro ao resumir texto."); 
                    println!("\nResumo:\n{}\n", response);
                } else {
                    eprintln!("\nErro ao ler arquivo. Tente novamente.\n")
                }
                
                let mut enter = String::new();
                print!("Pressine Enter para continuar");
                let _ = stdout().flush();
                stdin().read_line(&mut enter).unwrap();
            }
            _ => {}
        }
    }
}