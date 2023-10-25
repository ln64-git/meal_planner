use dotenv::dotenv;
use reqwest;
use serde_json::json;
use std::env;

pub async fn chat(role: &str, ingredients: Vec<&str>) -> Result<(), reqwest::Error> {
    dotenv().ok();
    let client = reqwest::Client::new();
    let api_key = env::var("API_KEY").expect("API_KEY not found.");
    let content = ingredients.join(", ");
    let request_data =
        json!({
        "model": "gpt-3.5-turbo",
        "messages": [
            {
                "role": "system",
                "content": "You are a master chef who specializes in simplicity of meals around the world for the average person."
            },
            {
                "role": role.to_string(),
                "content": format!("Build me a recipe with the following ingredients, {}.", content)
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
