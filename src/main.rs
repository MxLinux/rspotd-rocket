#[macro_use]
extern crate rocket;

use rocket_dyn_templates::Template;
use rspotd::DEFAULT_SEED;
use serde_json::json;

mod forms;

#[get("/")]
fn index() -> Template {
    let context = json!({
        "title": "ARRIS/CommScope Password of the Day Generator",
        "default_seed": DEFAULT_SEED
    });
    return Template::render("index", &context);
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .attach(Template::fairing())
}
