use std::{io, vec};

use rocket::{http::hyper::Error, response::status::NotFound, serde::json::Json};
use serde::{Deserialize, Serialize};

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello world"
}

#[derive(Deserialize, Serialize)]
struct User {
    name: String,
    age: u128,
}

#[get("/teste", format = "json")]
fn teste() -> Json<User> {
    let user = User {
        name: "Gustavo".to_string(),
        age: 340282366920938463463374607431768211455,
    };

    Json(User {
        name: user.name,
        age: user.age,
    })
}

#[launch]
fn start_server() -> _ {
    rocket::build().mount("/", routes![index, teste])
}
