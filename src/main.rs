#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate rocket_contrib;

mod chat {
    pub(crate) mod gpt_3_5_turbo;
    pub(crate) mod meal;
}

mod routes {
    pub(crate) mod user;
    pub(crate) mod meal;
}
use routes::user::static_rocket_route_info_for_signin;
use routes::meal::static_rocket_route_info_for_get_meal;

fn main() {
    rocket::ignite().mount("/", routes![signin]).launch();
    rocket::ignite().mount("/meal", routes![get_meal]).launch();
}
