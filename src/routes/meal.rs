use rocket_contrib::json::Json;
use serde::Serialize;

use crate::chat::gpt_3_5_turbo;

#[derive(Serialize)]
pub struct RecipeResponse {
    title: String,
    ingredients: Vec<String>,
    instructions: Vec<String>,
}

#[post("/meal")]
pub fn get_meal() -> Json<RecipeResponse> {
    let ingredients = vec!["potato", "lamb", "cinnamon", "onion"];

    // let response_str: String = chat(ingredients).await?;
    // let response: ChatCompletion = serde_json::from_str(&response_str)?;

    let response = tokio::runtime::Runtime
        ::new()
        .unwrap()
        .block_on(gpt_3_5_turbo::chat(ingredients));

    match response {
        Ok(response) => {
            println!("{}", response);
            // let recipe = meal::get_recipe(&response);
            // let recipe_response = RecipeResponse {
            //     title: recipe.title.clone(),
            //     ingredients: recipe.ingredients.clone(),
            //     instructions: recipe.instructions.clone(),
            // };

            let dummy_response = RecipeResponse {
                title: "Dummy Title".to_string(),
                ingredients: vec!["Dummy Ingredient".to_string()],
                instructions: vec!["Dummy Instruction".to_string()],
            };
            Json(dummy_response)
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
