#![allow(unused_imports)]
#[macro_use] extern crate rocket;

use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::{Json, json, Value};
use lazy_static::lazy_static;

lazy_static! {
    static ref BARBARIAN_CACHE: Value = {
        let path = "data/barbarian.json";
        let file = std::fs::File::open(path).unwrap();
        serde_json::from_reader(file).unwrap()
    };
}

#[get("/barbarian")]
fn get_barbarian() -> Value {
    json!(&*BARBARIAN_CACHE)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![
        get_barbarian,
    ])
}
