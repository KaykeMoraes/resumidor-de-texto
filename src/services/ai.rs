use std::env;
use dotenv::dotenv;
use serde_json::{Error, json};

pub struct AI {
    pub api_key: String,
    pub api_url: String,
}

impl AI {
    pub fn new() -> Self {
        dotenv().ok();
        AI { 
            api_key: env::var("API_KEY").expect("API_KEY inválida"), 
            api_url: env::var("API_URL").expect("API_URL inválida"), 
        }
    }
}

pub async fn summarize_text(prompt: String, ai: &AI) -> Result<String, reqwest::Error> {
    println!("Using URL: {}", &ai.api_url);
    let chat_request = json!({
        "model": "tngtech/deepseek-r1t2-chimera:free",
        "messages": [
              {
                "role": "user",
                "content": prompt
              }
            ]
    });
    
    let client = reqwest::Client::new();
    let response = client.post(&ai.api_url)
        .header("Authorization", format!("Bearer {}", &ai.api_key))
        .json(&chat_request)
        .send()
        .await?
        .text()
        .await?
    ;
    
    println!("Using URL: {}", &ai.api_url);
    
    Ok(response)
}