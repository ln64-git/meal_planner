mod gpt_3_5_turbo;
use gpt_3_5_turbo::chat;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    const ROLE: &str = "user";
    const CONTENT: &str =
        "what are some examples of programs I can write to utilize chatGPT api with a rust back";
    chat(ROLE, CONTENT).await?;
    Ok(())
}
