mod gpt_3_5_turbo;
mod recipe;
use gpt_3_5_turbo::chat;
use recipe::get_recipe;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ingredients = vec!["eggs", "chorizo", "beans", "hot sause"];
    let response = chat(ingredients).await?;
    let recipe = get_recipe(&response);

    if !recipe.title.is_empty() {
        println!("Recipe:");
        println!("{}", recipe.title);
        println!();
    }
    if !recipe.ingredients.is_empty() {
        println!("Ingredients:");
        for ingredient in recipe.ingredients {
            println!("• {}", ingredient);
        }
        println!();
    }
    if !recipe.instructions.is_empty() {
        println!("Instructions:");
        for (index, instruction) in recipe.instructions.iter().enumerate() {
            println!("{}. {}", index + 1, instruction);
        }
        println!();
    }
    if !recipe.note.is_empty() {
        println!("Note:");
        println!("{}", recipe.note);
        println!();
    }
    Ok(())
}
