#[macro_use]
extern crate rocket;

use rocket::form::{self, Form, Errors};
use rocket::fs::{FileServer,relative};
use rocket_dyn_templates::Template;
use rspotd::DEFAULT_SEED;
use serde_json::json;
use std::path::{Path, PathBuf};

mod functions;
// mod forms;

#[get("/")]
async fn index() -> Template {
    let date: String = functions::get_date_as_ymd();
    let context = json!({
        "title": "ARRIS/CommScope Password of the Day Generator",
        "date": date,
        "default_seed": DEFAULT_SEED
    });
    return Template::render("index", &context);
}

#[get("/?<date>&<date_begin>&<date_end>&<seed>")]
async fn index_submission(date: Option<String>, date_begin: Option<String>, date_end: Option<String>, seed: Option<String>) -> Template {
    let app_date: String = if date == None {
        functions::get_date_as_ymd()
    } else {
        date.unwrap()
    };
    let context = json!({
        "title": "ARRIS/CommScope Password of the Day Generator",
        "date": app_date,
        "default_seed": DEFAULT_SEED
    });
    return Template::render("index", &context);    
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index,index_submission])
        .mount("/static", FileServer::from(relative!("static")))
        .attach(Template::fairing())
}

// TODO:
// Checkbox for date range
// Style input to take less space
// Decide page style overall
// Implement all optional query parameters
// Seed validation - expose in library?
// Date validation - expose in library?
