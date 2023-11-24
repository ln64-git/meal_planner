use dotenv::dotenv;
use reqwest;
use serde::{ Deserialize, Serialize };
use serde_json::json;
use std::env;

#[derive(Debug, Deserialize, Serialize)]
pub struct RecipeResponse {
    pub id: String,
    pub object: String,
    pub created: i64,
    pub model: String,
    pub choices: Vec<Choice>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Choice {
    pub index: usize,
    pub message: Message,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}

pub async fn chat(ingredients: Vec<&str>) -> Result<RecipeResponse, reqwest::Error> {
    dotenv().ok();
    let client = reqwest::Client::new();
    let api_key = env::var("OPENAI_API_KEY").expect("API_KEY not found.");

    let request_data =
        json!({
        "model": "gpt-3.5-turbo",
        "messages": [
            {
                "role": "user",
                "content": format!(
                    "Complete the following recipe with minimal scientific fashion. 'Name', 'Ingredients' and 'Instructions' labeled. Ingredients: {}",
                    ingredients.join(", ")
                ),
            },
        ],
        "temperature": 0.7,
    });

    let response: RecipeResponse = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&request_data)
        .send().await?
        .json().await?;

    Ok(response)
}
