// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct ChatCompletion {
    // id: String,
    // object: String,
    // created: u64,
    // model: String,
        // choices: Vec<Choice>,
    // usage: Usage,
}

#[derive(Debug, Deserialize)]
struct Choice {
    // index: u64,
        // message: Message,
    // finish_reason: String,
}

#[derive(Debug, Deserialize)]
struct Message {
    // role: String,
        // content: String,
}

#[derive(Debug, Deserialize)]
struct Usage {
    // prompt_tokens: u64,
    // completion_tokens: u64,
    // total_tokens: u64,
}

mod chat {
    pub(crate) mod gpt_3_5_turbo;
    pub(crate) mod meal;
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![chat::gpt_3_5_turbo::get_recipe])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}


