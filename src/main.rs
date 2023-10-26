mod chat {
    pub(crate) mod gpt_3_5_turbo;
    pub(crate) mod recipe;
}
use chat::gpt_3_5_turbo::chat;
use chat::recipe::{ get_recipe, print_recipe };

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ingredients = vec!["honey", "bacon", "donuts", "lemon"];
    let response = chat(ingredients).await?;
    let recipe = get_recipe(&response);
    print_recipe(&recipe);
    Ok(())
}
