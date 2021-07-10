#[macro_use] extern crate rocket;

use rocket::fs::{FileServer, relative};
use rocket::serde::json::{Json, Value, json};
use rocket::serde::{Serialize, Deserialize};
use std::fs;

#[get("/sandwich")]
fn sandwich() -> &'static str {
    "🥪"
}

#[get("/taco")]
fn taco() -> &'static str {
    "🌮"
}

#[get("/problems/list")]
fn list_problems() -> Value {
    json!({"files": directory_files("../problems")})
}

#[get("/solutions/list")]
fn list_solutions() -> Value {
    json!({"files": directory_files("../solutions")})
}

fn directory_files(path: &str) -> Vec<String> {
    let paths = fs::read_dir(path).unwrap();
    let mut result = vec![];

    for path in paths{
        if let Ok(path) = path {
            if path.file_type().unwrap().is_file() {
                let name = path.file_name().as_os_str().to_str().unwrap().to_string();
                if name.ends_with(".json") {
                    result.push(name);
                }
            }
        }
    }
    result
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![sandwich])
        .mount("/", routes![taco])
        .mount("/", routes![list_problems])
        .mount("/", routes![list_solutions])
        .mount("/problems", FileServer::from(relative!("../problems")))
        .mount("/solutions", FileServer::from(relative!("../solutions")))
        .mount("/", FileServer::from(relative!("../web")).rank(11))
}
