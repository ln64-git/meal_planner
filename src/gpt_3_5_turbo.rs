use chatgpt::*;
use dotenv::dotenv;
use std::env;
use chatgpt::prelude::*;

use crate::recipe::get_recipe;

pub async fn chat(ingredients: Vec<&str>) -> Result<()> {
    dotenv().ok();
    let api_key = env::var("API_KEY").expect("API_KEY not found.");
    let client = ChatGPT::new(api_key)?;
    let content: String =
        "Complete the following recipe with minimal scientific fashion, ingredients and instructions labeled.".to_owned() +
        &ingredients.join(", ");

    let response = client.send_message(content.clone()).await?;
    get_recipe(&response);
    Ok(())
}
