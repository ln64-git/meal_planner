#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate rocket_contrib;

mod routes {
    pub(crate) mod user;
}
use routes::user::static_rocket_route_info_for_signin;

fn main() {
    rocket::ignite().mount("/", routes![signin]).launch();
}
