use rocket_contrib::json::Json;
use serde::Serialize;

use crate::chat::{ gpt_3_5_turbo, meal };

#[derive(Serialize)]
pub struct RecipeResponse {
    title: String,
    ingredients: Vec<String>,
    instructions: Vec<String>,
}

#[post("/meal")]
pub fn get_meal() -> Json<RecipeResponse> {
    let ingredients = vec!["potato", "lamb", "cinnamon", "onion"];
    let response = tokio::runtime::Runtime
        ::new()
        .unwrap()
        .block_on(gpt_3_5_turbo::chat(ingredients));

    match response {
        Ok(response) => {
            let recipe = meal::get_recipe(&response);
            let recipe_response = RecipeResponse {
                title: recipe.title.clone(),
                ingredients: recipe.ingredients.clone(),
                instructions: recipe.instructions.clone(),
            };

            Json(recipe_response)
        }
        Err(error) => {
            print!("{}", error);
            Json(RecipeResponse {
                title: String::new(),
                ingredients: Vec::new(),
                instructions: Vec::new(),
            })
        }
    }
}
