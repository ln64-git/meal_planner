use chatgpt::types::CompletionResponse;
use regex::Regex;

pub fn get_recipe(response: &CompletionResponse) -> RecipeInfo {
    let mut in_ingredients = false;
    let mut in_instructions = false;

    let mut title = String::new(); // Changed the name from 'recipe' to 'title'
    let mut note = String::new();
    let mut ingredients = Vec::new();
    let mut instructions = Vec::new();

    for message_choice in &response.message_choices {
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

    // Remove extra whitespace from the title and note
    title = title.trim().to_string();
    note = note.trim().to_string();

    RecipeInfo {
        title, // Updated the field name
        note, // Added note field
        ingredients,
        instructions,
    }
}

pub struct RecipeInfo {
    pub title: String, // Changed the name from 'recipe' to 'title'
    pub note: String, // Added note field
    pub ingredients: Vec<String>,
    pub instructions: Vec<String>,
}
