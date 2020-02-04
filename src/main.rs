#![feature(proc_macro_hygiene)]
#![feature(decl_macro)]

use rocket::{routes};

mod router;

fn main() {
    rocket::ignite()
        .mount("/", routes![router::hello, router::search])
        .launch();
}
