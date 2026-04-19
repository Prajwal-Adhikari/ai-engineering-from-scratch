use reqwest::Client;
use serde_json::json;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();

    let token = env::var("HUGGINGFACE_TOKEN")
        .expect("❌ HUGGINGFACE_TOKEN not set");

    // ✅ Correct base URL per hf-inference docs
    let url = "https://router.huggingface.co/v1/chat/completions";

    let body = json!({
        "model": "katanemo/Arch-Router-1.5B:hf-inference",
        "messages": [
            {
                "role": "user",
                "content": "What is a neural network in one sentence?"
            }
        ],
        "max_tokens": 200
    });

    println!("Sending request...");

    let response = client
        .post(url)
        .bearer_auth(token)
        .json(&body)
        .send()
        .await?;

    let status = response.status();
    let json: serde_json::Value = response.json().await?;

    println!("Status: {}", status);

    if let Some(content) = json["choices"][0]["message"]["content"].as_str() {
        println!("Answer: {}", content);
    } else {
        println!("Full response: {}", serde_json::to_string_pretty(&json)?);
    }

    Ok(())
}