// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod gpt_3_5_turbo;
// mod parse_object;

fn main() {
    tauri::Builder
        ::default()
        .invoke_handler(tauri::generate_handler![gpt_3_5_turbo::get_recipe])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    // const STRING1: &str =
    //     "Recipe: Cheesy Chicken Pasta

    //     Ingredients:
    //     - 2 chicken breasts, boneless and skinless
    //     - 1 cup shredded cheese (cheddar, mozzarella, or your choice)
    //     - 8 oz pasta (penne, fusilli, or your choice)
    //     - Salt, to taste
    //     - Pepper, to taste
    //     - Olive oil

    //     Instructions:
    //     1. Bring a large pot of water to a boil. Add a pinch of salt and cook the pasta according to the package instructions until al dente. Drain and set aside.
    //     2. While the pasta is cooking, season the chicken breasts with salt and pepper on both sides.
    //     3. Heat a drizzle of olive oil in a large skillet over medium-high heat. Add the chicken breasts and cook for about 6-8 minutes per side, or until they are cooked through and no longer pink in the center. Remove the chicken from the skillet and let it rest for a few minutes.
    //     4. Slice the cooked chicken breasts into thin strips or bite-sized pieces.
    //     5. In the same skillet, add the cooked pasta and the sliced chicken. Stir them together over low heat to combine.
    //     6. Sprinkle the shredded cheese over the pasta and chicken mixture. Cover the skillet with a lid or foil and let it cook for a few minutes until the cheese melts.
    //     7. Remove the skillet from the heat and let it rest for a couple of minutes to allow the flavors to meld together.
    //     8. Serve the cheesy chicken pasta hot and enjoy!

    //     Note: Feel free to add any additional seasonings or vegetables of your choice to enhance the flavor.";
    // const STRING2: &str =
    //     "Name: Limeade

    //     Ingredients:
    //     - 2 limes
    //     - 1 cup water
    //     - 1 pinch of salt
    //     - 2 tablespoons sugar

    //     Instructions:
    //     1. Slice the limes in half and squeeze out the juice into a pitcher.
    //     2. Add water to the pitcher and stir well.
    //     3. Sprinkle in a pinch of salt.
    //     4. Pour in the sugar and stir until it dissolves completely.
    //     5. Taste the limeade and adjust the sugar or lime juice according to your preference.
    //     6. Add ice cubes if desired and serve chilled.

    //     Enjoy your refreshing limeade!";
    // const STRING3: &str =
    //     "Name: Grilled Cheeseburger Sandwich

    //     Ingredients:
    //     - 2 slices of bread
    //     - 1-2 slices of cheese (such as cheddar or American cheese)
    //     - 1 beef patty (precooked or freshly cooked)

    //     Instructions:
    //     1. Heat a non-stick skillet or griddle over medium heat.
    //     2. If using a precooked beef patty, heat it according to the package instructions. If using a freshly cooked beef patty, ensure it is fully cooked and hot.
    //     3. Place one slice of bread on a clean surface or plate.
    //     4. Layer the cheese slices on top of the bread slice.
    //     5. Carefully place the cooked beef patty on top of the cheese.
    //     6. Place the second slice of bread on top of the beef patty to complete the sandwich.
    //     7. Gently transfer the sandwich to the preheated skillet or griddle.
    //     8. Cook the sandwich for approximately 2-3 minutes on each side, or until the bread is golden brown and the cheese is melted.
    //     9. Carefully flip the sandwich using a spatula and cook the other side for an additional 2-3 minutes.
    //     10. Once both sides are golden brown and the cheese is fully melted, remove the sandwich from the skillet or griddle.
    //     11. Allow the sandwich to cool for a minute or two before serving.
    //     12. Cut the sandwich in half, if desired, and enjoy your delicious grilled cheeseburger sandwich!";
    // let parsed_recipe1 = parse_object::parse_recipe(STRING1);
    // let parsed_recipe2 = parse_object::parse_recipe(STRING2);
    // let parsed_recipe3 = parse_object::parse_recipe(STRING3);
    // println!("{:#?}", parsed_recipe1);
    // println!("{:#?}", parsed_recipe2);
    // println!("{:#?}", parsed_recipe3);
}
