use regex::Regex;
use serde::{ Serialize, Deserialize };

use crate::ChatCompletion;

#[derive(Debug, Serialize, Deserialize)]
pub struct RecipeInfo {
    pub title: String,
    pub note: String,
    pub ingredients: Vec<String>,
    pub instructions: Vec<String>,
}

pub(crate) fn get_recipe(response: &ChatCompletion) -> RecipeInfo {
    let mut in_ingredients = false;
    let mut in_instructions = false;
    let mut title = String::new();
    let mut note = String::new();
    let mut ingredients = Vec::new();
    let mut instructions = Vec::new();

    for message_choice in &response.choices {
        for line in message_choice.message.content.lines() {
            if !line.trim().is_empty() {
                if line.trim() == "Ingredients:" {
                    in_ingredients = true;
                    in_instructions = false;
                    continue;
                } else if line.trim() == "Instructions:" {
                    in_ingredients = false;
                    in_instructions = true;
                    continue;
                }
                if line.starts_with("Recipe:") {
                    title = line[7..].trim().to_string(); // Capture the recipe title without the label
                } else if line.starts_with("Note:") {
                    note = line[5..].trim().to_string(); // Capture the note without the label
                } else {
                    let number_pattern = Regex::new(r"^\d+\.\s").unwrap();
                    if in_ingredients {
                        let ingredient = line.trim().trim_start_matches("-"); // Remove "- " and "• " from the beginning
                        ingredients.push(ingredient.to_string());
                    } else if in_instructions {
                        let cleaned_instruction = number_pattern.replace(line, "");
                        instructions.push(cleaned_instruction.trim().to_string());
                    }
                }
            }
        }
    }
    title = title.trim().to_string();
    note = note.trim().to_string();
    RecipeInfo {
        title,
        note,
        ingredients,
        instructions,
    }
}

pub fn print_recipe(recipe: &RecipeInfo) {
    if !recipe.title.is_empty() {
        println!("Recipe:");
        println!("{}", recipe.title);
        println!();
    }
    if !recipe.ingredients.is_empty() {
        println!("Ingredients:");
        for ingredient in &recipe.ingredients {
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
}