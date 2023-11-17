mod gpt_3_5_turbo;
mod recipe;
use gpt_3_5_turbo::chat;
use recipe::get_recipe;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct ChatCompletion {
    // id: String,
    // object: String,
    // created: u64,
    // model: String,
    choices: Vec<Choice>,
    // usage: Usage,
}

#[derive(Debug, Deserialize)]
struct Choice {
    // index: u64,
    message: Message,
    // finish_reason: String,
}

#[derive(Debug, Deserialize)]
struct Message {
    // role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct Usage {
    // prompt_tokens: u64,
    // completion_tokens: u64,
    // total_tokens: u64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ingredients = vec!["ramen", "peanut butter", "olive oil", "cheese"];
    let response_str: String = chat(ingredients).await?;
    let response: ChatCompletion = serde_json::from_str(&response_str)?;
    get_recipe(&response);
    Ok(())
}
