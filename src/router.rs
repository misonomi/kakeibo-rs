use rocket::{get};


#[get("/")]
pub fn home() -> &'static str {
    "the home page with login function"
}

#[get("/add")]
pub fn add() -> &'static str {
    "input receipt"
}

#[get("/statistics")]
pub fn statistics() -> &'static str {
    "statistics"
}
