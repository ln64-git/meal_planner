use dotenv::dotenv;
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::env;

#[derive(Debug, Deserialize, Serialize)]
pub struct RecipeResponse {
    id: String,
    object: String,
    created: i64,
    model: String,
    choices: Vec<Choice>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Choice {
    index: usize,
    message: Message,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Message {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Recipe {
    pub name: String,
    pub ingredients: Vec<String>,
    pub instructions: Vec<String>,
}
impl RecipeResponse {
    pub fn to_recipe(&self) -> Result<Recipe, &'static str> {
        let message_content = self
            .choices
            .get(0)
            .map(|choice| choice.message.content.clone())
            .unwrap_or_default();

        // Extracting relevant portion of the message content
        let start_index = message_content.find("Recipe Name:").unwrap_or(0);
        let end_index = message_content.find("Instructions:").unwrap_or(message_content.len());

        let relevant_content = &message_content[start_index..end_index];

        // Debug print the relevant content
        println!("{}", relevant_content);

        // Attempt to deserialize and print the JSON
        match serde_json::from_str::<Recipe>(&relevant_content) {
            Ok(recipe) => {
                println!("Parsed JSON: {:#?}", recipe);
                Ok(recipe)
            }
            Err(_) => Err("Failed to parse recipe"),
        }
    }
}

#[tauri::command]
pub async fn get_recipe(ingredients: Vec<&str>) -> Result<Recipe, &'static str> {
    println!("Function called");
    let result = chat(ingredients).await;
    match result {
        Ok(result) => result.to_recipe(),
        Err(_) => Err("Failed to get recipe"),
    }
}

pub async fn chat(ingredients: Vec<&str>) -> Result<RecipeResponse, reqwest::Error> {
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

    let response: RecipeResponse = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&request_data)
        .send()
        .await?
        .json()
        .await?;

    Ok(response)
}
