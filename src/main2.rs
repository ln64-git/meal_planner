mod chat {
    pub(crate) mod gpt_3_5_turbo;
    pub(crate) mod meal;
}
use chat::gpt_3_5_turbo::chat;
use chat::meal::{ get_recipe, print_recipe };

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ingredients = vec!["potato", "lamb", "cinnamon", "onion"];
    let response = chat(ingredients).await?;
    let recipe = get_recipe(&response);
    print_recipe(&recipe);
    Ok(())
}
