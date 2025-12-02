use std::{env};
use anyhow::{Error, Ok};
use dotenv::dotenv;
use gemini_rust::{ClientError, Gemini};

fn new_client() -> Result<Gemini, ClientError> {
    dotenv().ok();
    let api_key = env::var("API_KEY").expect("Error reading the api key");
    
    Gemini::new(api_key)    
}

pub async fn send_prompt(prompt: &String) -> Result<String, Error>{
    let client = new_client().unwrap();
    
    let response = client
        .generate_content()
        .with_system_prompt("Você é um resumidor de textos. O usuário enviará um arquivo ou um pedido para você sobre algum texto, livro, artigo e etc; e você retornará um resumo de simples leitura e entendimento")
        .with_user_message(prompt)
        .execute()
        .await?;

    Ok(response.text())
}