use rocket::{get, State, http::RawStr};
use rocket_contrib::json::Json;


#[get("/")]
pub fn home() -> &'static str {
    "the home page with login function"
}

#[get("/add")]
pub fn search() -> &'static str {
    "input receipt"
}

#[get("/statistics")]
pub fn search() -> &'static str {
    "statistics"
}
