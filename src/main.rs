#![feature(proc_macro_hygiene, decl_macro)]

use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct ChatCompletion {
    // id: String,
    // object: String,
    // created: u64,
    // model: String,
    choices: Vec<Choice>,
    // usage: Usage,
}

#[derive(Debug, Deserialize)]
struct Choice {
    // index: u64,
    message: Message,
    // finish_reason: String,
}

#[derive(Debug, Deserialize)]
struct Message {
    // role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct Usage {
    // prompt_tokens: u64,
    // completion_tokens: u64,
    // total_tokens: u64,
}



#[macro_use]
extern crate rocket;
extern crate rocket_contrib;

mod chat {
    pub(crate) mod gpt_3_5_turbo;
    pub(crate) mod meal;
}

mod routes {
    pub(crate) mod meal;
}
use routes::meal::static_rocket_route_info_for_get_meal;

fn main() {
    rocket::ignite().mount("/", routes![ get_meal]).launch();
}
