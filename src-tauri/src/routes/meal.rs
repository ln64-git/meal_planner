use crate::ChatCompletion;
use rocket_contrib::json::Json;
use serde::Serialize;

use crate::chat::{gpt_3_5_turbo, meal};

#[derive(Serialize)]
pub struct RecipeResponse {
    title: String,
    ingredients: Vec<String>,
    instructions: Vec<String>,
}

#[post("/meal")]
pub fn get_meal() -> Json<RecipeResponse> {
    let ingredients = vec!["potato", "lamb", "cinnamon", "onion"];

    let response_str: String = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(gpt_3_5_turbo::chat(ingredients))
        .unwrap_or_else(|err| {
            eprintln!("Error fetching response: {}", err);
            String::new() // Return an empty string on error or handle it differently
        });

    let response: Result<ChatCompletion, _> = serde_json::from_str(&response_str);

    match response {
        Ok(chat_completion) => {
            let recipe = meal::get_recipe(&chat_completion);

            let recipe_response = RecipeResponse {
                title: recipe.title.clone(),
                ingredients: recipe.ingredients.clone(),
                instructions: recipe.instructions.clone(),
            };
            Json(recipe_response)
        }
        Err(error) => {
            eprintln!("Error deserializing response: {}", error);
            Json(RecipeResponse {
                title: String::new(),
                ingredients: Vec::new(),
                instructions: Vec::new(),
            })
        }
    }
}
