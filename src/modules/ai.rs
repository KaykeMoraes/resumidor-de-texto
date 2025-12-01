use std::env;
use dotenv::dotenv;
use serde::Deserialize;
use serde_json::{json};

pub struct AI {
    pub api_key: String,
    pub api_url: String,
}

#[derive(Deserialize, Debug)]
pub struct Part {
    pub text: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Content {
    pub parts: Vec<Part>,
}

#[derive(Deserialize, Debug)]
pub struct Candidate {
    pub content: Content,
}

#[derive(Deserialize, Debug)]
struct AIResponse {
    pub candidates: Option<Vec<Candidate>>
}

impl Default for AI {
    fn default() -> Self {
        Self::new()
    }
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

pub async fn summarize_text(prompt: &String, ai: &AI) -> Result<String, anyhow::Error> {
    let chat_request = json!(
        {
            "system_instruction": {
                "parts": [
                    {
                        "text": "Você é um resumidor de textos. Retorne sempre um texto que seja não tão grande e de fácil leitura."
                    }
                ]
            },
            "contents": [
                {
                    "parts": [
                        {
                            "text": &prompt
                        }
                    ]
                }
            ]
        }
    );
    
    let client = reqwest::Client::new();
    let response = client.post(&ai.api_url)
        .query(&[("key", &ai.api_key)])
        .json(&chat_request)
        .send()
        .await?
        .json::<AIResponse>()
        .await?
    ;
    
    if let Some(candidates) = response.candidates
        && let Some(text) = candidates.first()
            .and_then(|c| c.content.parts.first())
            .and_then(|p| p.text.clone())
    {
        return Ok(text)
    } 
    
    Err(
        anyhow::anyhow!("Erro ao se conectar com a IA!")
    )
}