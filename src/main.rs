use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
struct Part {
    text: String,
}

#[derive(Deserialize, Debug)]
struct Content {
    parts: Vec<Part>,
}

#[derive(Deserialize, Debug)]
struct Candidate {
    content: Content,
}

#[derive(Deserialize, Debug)]
struct ApiResponse {
    candidates: Vec<Candidate>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let contents = vec![HashMap::from([(
        "parts".to_string(),
        vec![HashMap::from([(
            "text".to_string(),
            "Explain how AI works".to_string(),
        )])],
    )])];

    let map = HashMap::from([("contents".to_string(), contents)]);

    const API_KEY: &str = "";

    let client = reqwest::Client::new();
    let response = client
        .post(format!(
            "https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash-latest:generateContent?key={}",
            API_KEY
        ))
        .json(&map)
        .send()
        .await?;

    if response.status().is_success() {
        let api_response: ApiResponse = response.json().await?;
        if let Some(candidate) = api_response.candidates.first() {
            if let Some(part) = candidate.content.parts.first() {
                println!("First part text: {}", part.text);
            } else {
                println!("No parts found.");
            }
        } else {
            println!("No candidates found.");
        }
    } else {
        println!("Request failed with status: {}", response.status());
    }

    Ok(())
}
