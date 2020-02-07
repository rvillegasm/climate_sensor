#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate rocket_cors;
extern crate bson;
extern crate mongodb;
extern crate serde;

mod databases;
mod handlers;
mod models;

use rocket::http::Method;
use rocket_cors::{
    AllowedHeaders, AllowedOrigins,
    Cors, CorsOptions,
};

fn make_cors() -> Cors {
    let allowed_origins = AllowedOrigins::some_exact(&[
        "http://localhost:8080",
    ]);

    CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::some(&[
            "Authorization",
            "Accept",
            "Access-Control-Allow-Origin",
        ]),
        allow_credentials: true,
        ..Default::default()
    }.to_cors()
    .expect("Error While Building Cors!")
}

fn main() {
    rocket::ignite()
        .mount(
            "/climate",
            routes![handlers::sensor::create_entry, handlers::user::get_data],
        )
        .attach(make_cors())
        .launch();
}
