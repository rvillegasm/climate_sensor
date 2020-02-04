#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate bson;
extern crate mongodb;
extern crate serde;

// mod sensors;
// mod users;

mod databases;
mod handlers;
mod models;

fn main() {
    rocket::ignite()
        .mount(
            "/climate",
            routes![handlers::sensor::create_entry, handlers::user::get_data],
        )
        .launch();
}
