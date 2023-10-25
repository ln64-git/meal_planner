mod gpt_3_5_turbo;
use gpt_3_5_turbo::chat;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ingredients = vec!["cheese", "bread", "butter"];
    chat(ingredients).await?;
    Ok(())
}
