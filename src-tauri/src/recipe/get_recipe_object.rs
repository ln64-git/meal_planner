// The rest of your imports and code
use crate::recipe::get_recipe_call::{ chat, RecipeResponse };

use serde::{ Deserialize, Serialize };
use tauri::regex::Regex;

#[derive(Debug, Deserialize, Serialize)]
pub struct Recipe {
    pub name: String,
    pub ingredients: Vec<String>,
    pub instructions: Vec<String>,
    pub note: Option<String>,
}

impl RecipeResponse {
    pub fn to_recipe(&self) -> Result<Recipe, &'static str> {
        let message_content = self.choices
            .get(0)
            .map(|choice| choice.message.content.clone())
            .unwrap_or_default();

        // Debug print the relevant content
        println!("{}", message_content);
        println!("{:?}", parse_recipe(&message_content));

        // Attempt to deserialize and print the JSON
        match parse_recipe(&message_content) {
            Some(recipe) => {
                println!("Parsed Recipe: {:?}", recipe);
                Ok(recipe)
            }
            None => Err("Failed to parse recipe"),
        }
    }
}

#[tauri::command]
pub async fn get_recipe(ingredients: Vec<&str>) -> Result<Recipe, &'static str> {
    println!("Function called");
    let result = chat(ingredients).await;
    match result {
        Ok(result) => result.to_recipe(),
        Err(_) => Err("Failed to get recipe"),
    }
}

pub fn parse_recipe(input: &str) -> Option<Recipe> {
    // Define regular expressions for name, ingredients, instructions, and optional note
    let name_regex: Regex = Regex::new(r"^(?:(?:Name|Recipe): ?)?(.+?)(?:\n|$)").unwrap();
    let ingredients_regex = Regex::new(r"Ingredients:\n((?s).*?)\nInstructions:").unwrap();
    let instructions_regex = Regex::new(r"Instructions:\n((?s).*?)(?:\nNote:|$)").unwrap();
    let note_regex = Regex::new(r"Note:\n((?s).*)$").unwrap();

    // Extract name, ingredients, instructions, and optional note using regular expressions
    let name = name_regex
        .captures(input)?
        .get(1)
        .map_or("", |m| m.as_str().trim())
        .to_string();
    let ingredients_str = ingredients_regex.captures(input)?.get(1)?.as_str();
    let instructions_str = instructions_regex.captures(input)?.get(1)?.as_str();
    let note_str = note_regex
        .captures(input)
        .map(|capture| capture.get(1).unwrap().as_str().trim().to_string());

    // Split ingredients and instructions into Vec<String>, removing decorative syntax
    let ingredients = ingredients_str
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line|
            line
                .trim_start_matches(
                    |c: char| (c.is_whitespace() || c.is_digit(10) || c.is_ascii_punctuation())
                )
                .to_string()
        )
        .collect();

    let instructions = instructions_str
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line|
            line
                .trim_start_matches(
                    |c: char| (c.is_whitespace() || c.is_digit(10) || c.is_ascii_punctuation())
                )
                .to_string()
        )
        .collect();

    // Create Recipe struct
    let recipe = Recipe {
        name,
        ingredients,
        instructions,
        note: note_str,
    };

    Some(recipe)
}
