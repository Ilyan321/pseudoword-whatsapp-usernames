use reqwest::blocking::Client;
use serde::Deserialize;
use serde_json::json;
use std::time::Duration;
use crate::prompt::MASTER_PROMPT;

#[derive(Deserialize)]
struct ChatChoice {
    message: ChatMessage,
}

#[derive(Deserialize)]
struct ChatMessage {
    content: String,
}

#[derive(Deserialize)]
struct ChatResponse {
    choices: Vec<ChatChoice>,
}

pub fn generate_with_groq(api_key: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::builder()
        .timeout(Duration::from_secs(30))
        .build()?;

    let body = json!({
        "model": "llama-3.3-70b-versatile",
        "messages": [
            {
                "role": "system",
                "content": MASTER_PROMPT
            },
            {
                "role": "user",
                "content": "Generate 2 completely different pseudoword WhatsApp usernames now adhering strictly to all laws."
            }
        ],
        "temperature": 1.1
    });

    let res = client
        .post("https://api.groq.com/openai/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&body)
        .send()?;

    if !res.status().is_success() {
        let err_text = res.text().unwrap_or_default();
        return Err(format!("Groq API error (status {}): {}", res.status(), err_text).into());
    }

    let parsed: ChatResponse = res.json()?;
    if let Some(choice) = parsed.choices.into_iter().next() {
        Ok(choice.message.content.trim().to_string())
    } else {
        Err("No choices returned from Groq API".into())
    }
}
