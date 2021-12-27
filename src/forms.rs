use rocket::form::{self, Errors};
use rspotd::DEFAULT_SEED;

#[derive(FromForm)]
struct POTDForm {
    #[field(default = DEFAULT_SEED)]
    seed: String,
    date: String    
}