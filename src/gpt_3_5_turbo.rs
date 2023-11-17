use dotenv::dotenv;
use reqwest;
use serde_json::json;
use std::env;

pub async fn chat(ingredients: Vec<&str>) -> Result<String, reqwest::Error> {
    dotenv().ok();
    let client = reqwest::Client::new();
    let api_key = env::var("OPENAI_API_KEY").expect("API_KEY not found.");

    let request_data = json!({
        "model": "gpt-3.5-turbo",
        "messages": [
            {
                "role": "user",
                "content": format!(
                    "Complete the following recipe with minimal scientific fashion. Name, ingredients and instructions labeled, add optional ingredients to round meal out. Ingredients: {}",
                    ingredients.join(", ")
                ),
            },
        ],
        "temperature": 0.7,
    });

    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&request_data)
        .send()
        .await?;

        let body = response.text().await?;
        Ok(body)
}
