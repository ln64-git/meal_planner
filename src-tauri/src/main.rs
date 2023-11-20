// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod gpt_3_5_turbo;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![gpt_3_5_turbo::get_recipe])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}


