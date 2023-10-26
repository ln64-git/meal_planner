use core::fmt;

use rocket_contrib::json::Json;
use crate::chat::gpt_3_5_turbo;
use crate::chat::meal;
use crate::chat::meal::RecipeInfo;

impl fmt::Display for RecipeInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if !self.title.is_empty() {
            write!(f, "Recipe:\n{}", self.title)?;
        }
        if !self.ingredients.is_empty() {
            write!(f, "Ingredients:\n")?;
            for ingredient in &self.ingredients {
                write!(f, "• {}\n", ingredient)?;
            }
        }
        if !self.instructions.is_empty() {
            write!(f, "Instructions:\n")?;
            for (index, instruction) in self.instructions.iter().enumerate() {
                write!(f, "{}. {}\n", index + 1, instruction)?;
            }
        }
        if !self.note.is_empty() {
            write!(f, "Note:\n{}", self.note)?;
        }

        Ok(())
    }
}

#[post("/meal", data = "<recipe>")]
pub fn get_meal(recipe: Json<RecipeInfo>) -> Json<String> {
    let ingredients = vec!["potato", "lamb", "cinnamon", "onion"];

    // Use Tokio's runtime to call the async function
    let response = tokio::runtime::Runtime
        ::new()
        .unwrap()
        .block_on(gpt_3_5_turbo::chat(ingredients));

    match response {
        Ok(response) => {
            let recipe = meal::get_recipe(&response);
            Json(recipe.to_string())
        }
        Err(error) => { Json(format!("Error: {:?}", error)) }
    }
}
