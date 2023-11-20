use dotenv::dotenv;
use reqwest;
use serde_json::json;
use std::env;

#[tauri::command]
pub async fn get_recipe(ingredients: Vec<&str>) -> Result<String, &'static str> {
    println!("function called");
    let result = chat(ingredients).await;
    print!("{:?}", result);

    match result {
        Ok(result) => Ok(result),
        Err(_) => Err("Failed to get recipe"),
    }
}

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
