mod gpt_3_5_turbo;
mod recipe;
use gpt_3_5_turbo::chat;
use recipe::{ get_recipe, print_recipe };

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ingredients = vec!["eggs", "chorizo", "beans", "hot sause"];
    let response = chat(ingredients).await?;
    let recipe = get_recipe(&response);
    print_recipe(&recipe);
    Ok(())
}
