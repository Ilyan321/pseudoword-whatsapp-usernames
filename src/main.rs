mod config;
mod gemini;
mod groq;
mod prompt;

use config::{Config, Provider};
use std::io::{self, Write};

fn main() {
    println!("====================================================");
    println!("  🌌 Divine Pseudoword WhatsApp Username Generator  ");
    println!("====================================================");

    let mut config = Config::new();

    loop {
        let provider_name = match config.provider {
            Provider::Groq => "Groq (Llama 3.3 70B)",
            Provider::Gemini => "Gemini 2.5 Flash",
        };

        print!("\n⚡ Generating 2 pseudoword usernames using {}...", provider_name);
        io::stdout().flush().unwrap();

        let result = match config.provider {
            Provider::Groq => groq::generate_with_groq(&config.api_key),
            Provider::Gemini => gemini::generate_with_gemini(&config.api_key),
        };

        match result {
            Ok(output) => {
                println!("\n\n{}", output);
            }
            Err(e) => {
                eprintln!("\n❌ Error generating usernames: {}", e);
            }
        }

        println!("\n----------------------------------------------------");
        print!("Press [Enter] or 'r' to Retry, 'p' to switch Provider/Key, or 'q' to quit: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            break;
        }

        let trimmed = input.trim().to_lowercase();
        if trimmed == "q" || trimmed == "quit" || trimmed == "exit" {
            println!("Exiting. Stay divine!");
            break;
        } else if trimmed == "p" || trimmed == "k" {
            config = Config::prompt_user_setup();
        }
    }
}
