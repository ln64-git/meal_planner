use dotenv::dotenv;
use reqwest;
use serde_json::json;
use std::env;

pub async fn chat(role: &str, content: &str) -> Result<(), reqwest::Error> {
    dotenv().ok();
    let client = reqwest::Client::new();
    let api_key = env::var("API_KEY").expect("API_KEY not found.");
    let request_data =
        json!({
        "model": "gpt-3.5-turbo",
        "messages": [
            {
                "role": "system",
                "content": "You are clear and concise concentrated and meditated, do not respond with code."
            },
            {
                "role": role.to_string(),
                "content": content.to_string()
            }
        ],
        "temperature": 0.7
    });
    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&request_data)
        .send().await?;
    if response.status().is_success() {
        let body = response.text().await?;
        println!("Response body: {}", body);
    } else {
        println!("Request failed with status code: {}", response.status());
    }
    Ok(())
}
