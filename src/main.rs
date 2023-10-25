mod gpt_3_5_turbo;
mod recipe;
use gpt_3_5_turbo::chat;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ingredients = vec!["ramen", "peanut butter", "olive oil", "cheese"];
    chat(ingredients).await?;
    Ok(())
}
