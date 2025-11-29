use std::io::{self, Write, stdin};
use colored::{Color, Colorize};
use clear_screen::clear;
use resumidor_de_pdf::modules::{ai::{AI, summarize_text}, file_handle::{pdf::read_pdf, txt::read_txt}};

pub async fn run() {
    let ai = AI::new();
    loop {
        clear();
        println!("{}", "---------------RESUMIDOR DE TEXTO---------------".color(Color::BrightCyan));
        println!("Digite 0 para sair\nDigite 1 se quiser resumir texto de um pdf\nDigite 2 se quiser resumir um texto de um arquivo .txt");
        print!("Opcao: ");
        let _ = io::stdout().flush();
        
        let mut option = String::new();
        stdin().read_line(&mut option).expect("Erro ao ler a opcao.");
        
        let option: u8 = option.trim().parse().unwrap();
        
        match option {
            0 => {
                println!("Saindo..."); 
                break;
            } 
            1 => {
                print!("Digite o diretório do seu pdf: "); 
                io::stdout().flush();
                let mut path = String::new();
                stdin().read_line(&mut path).expect("Erro ao ler diretório");
                let text = read_pdf(&path.trim());
                let response = summarize_text(text.unwrap(), &ai);
                
                println!("\nResumo:\n{}\n", response.await.unwrap());
                
                let mut enter = String::new();
                print!("Pressine Enter para continuar");
                io::stdout().flush();
                stdin().read_line(&mut enter).unwrap();
            }
            2 => {
                print!("Digite o diretório do seu .txt: "); 
                io::stdout().flush();
                let mut path = String::new();
                stdin().read_line(&mut path).expect("Erro ao ler diretório");
                let text = read_txt(&path.trim());
                let response = summarize_text(text.unwrap(), &ai);
                
                println!("\nResumo:\n{}\n", response.await.unwrap());
                
                let mut enter = String::new();
                print!("Pressine Enter para continuar");
                io::stdout().flush();
                stdin().read_line(&mut enter).unwrap();
            }
            _ => {}
        }
    }
}