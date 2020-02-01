#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate serde;
extern crate mongodb;
extern crate bson;

mod sensors;

fn main() {
    rocket::ignite().mount("/climate", routes![sensors::create_entry]).launch();
}
