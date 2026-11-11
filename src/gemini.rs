use reqwest::blocking::Client;
use serde::Deserialize;
use serde_json::json;
use std::time::Duration;
use crate::prompt::MASTER_PROMPT;

#[derive(Deserialize)]
struct GeminiPart {
    text: Option<String>,
}

#[derive(Deserialize)]
struct GeminiContent {
    parts: Vec<GeminiPart>,
}

#[derive(Deserialize)]
struct GeminiCandidate {
    content: Option<GeminiContent>,
}

#[derive(Deserialize)]
struct GeminiResponse {
    candidates: Option<Vec<GeminiCandidate>>,
}

pub fn generate_with_gemini(api_key: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::builder()
        .timeout(Duration::from_secs(30))
        .build()?;

    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.5-flash:generateContent?key={}",
        api_key
    );

    let body = json!({
        "system_instruction": {
            "parts": [{ "text": MASTER_PROMPT }]
        },
        "contents": [
            {
                "role": "user",
                "parts": [{ "text": "Generate 2 completely different pseudoword WhatsApp usernames now adhering strictly to all laws." }]
            }
        ],
        "generationConfig": {
            "temperature": 1.1
        }
    });

    let res = client
        .post(&url)
        .header("Content-Type", "application/json")
        .json(&body)
        .send()?;

    if !res.status().is_success() {
        let err_text = res.text().unwrap_or_default();
        return Err(format!("Gemini API error (status {}): {}", res.status(), err_text).into());
    }

    let parsed: GeminiResponse = res.json()?;
    if let Some(candidates) = parsed.candidates {
        if let Some(candidate) = candidates.into_iter().next() {
            if let Some(content) = candidate.content {
                if let Some(part) = content.parts.into_iter().next() {
                    if let Some(text) = part.text {
                        return Ok(text.trim().to_string());
                    }
                }
            }
        }
    }

    Err("No valid text returned from Gemini API".into())
}
