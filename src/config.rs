use std::env;
use std::io::{self, Write};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Provider {
    Groq,
    Gemini,
}

pub struct Config {
    pub provider: Provider,
    pub api_key: String,
}

impl Config {
    pub fn new() -> Self {
        let groq_key = env::var("GROQ_API_KEY").unwrap_or_default();
        let gemini_key = env::var("GEMINI_API_KEY").unwrap_or_default();

        if !gemini_key.is_empty() {
            Config {
                provider: Provider::Gemini,
                api_key: gemini_key,
            }
        } else if !groq_key.is_empty() {
            Config {
                provider: Provider::Groq,
                api_key: groq_key,
            }
        } else {
            Self::prompt_user_setup()
        }
    }

    pub fn prompt_user_setup() -> Self {
        println!("\nSelect AI Provider:");
        println!("  1) Groq (Llama 3.3 70B Versatile)");
        println!("  2) Google Gemini (Gemini 2.5 Flash)");
        print!("Choose [1 or 2, default: 1]: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap_or_default();
        let choice = choice.trim();

        let provider = match choice {
            "2" => Provider::Gemini,
            _ => Provider::Groq,
        };

        let prompt_name = match provider {
            Provider::Groq => "Groq API Key (starts with gsk_)",
            Provider::Gemini => "Gemini API Key (starts with AIzaSy)",
        };

        print!("🔑 Enter your {}: ", prompt_name);
        io::stdout().flush().unwrap();

        let mut key = String::new();
        io::stdin().read_line(&mut key).unwrap_or_default();

        Config {
            provider,
            api_key: key.trim().to_string(),
        }
    }
}
