use chatgpt::*;
use dotenv::dotenv;
use std::env;
use chatgpt::prelude::*;

pub async fn chat(ingredients: Vec<&str>) -> Result<()> {
    dotenv().ok();
    let api_key = env::var("API_KEY").expect("API_KEY not found.");
    let client = ChatGPT::new(api_key)?;

    let initial_message =
        "You are a master chef who specializes in simplicity of meals around the world for the average person.";
    let content: String = ingredients.join(", ");
    let combined_message = format!(
        "{} Build me a recipe with the following ingredients, {}.",
        initial_message,
        content
    );

    let response = client.send_message(combined_message).await?;
    println!("Response: {}", response.message().content);
    Ok(())
}
