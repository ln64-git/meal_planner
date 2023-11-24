// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod recipe {
    pub(crate) mod get_recipe_call;
    pub(crate) mod get_recipe_object;
}

fn main() {
    tauri::Builder
        ::default()
        .invoke_handler(tauri::generate_handler![recipe::get_recipe_object::get_recipe])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
