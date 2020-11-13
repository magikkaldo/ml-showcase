#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

mod routes;

use rocket_contrib::serve::StaticFiles;

pub fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", StaticFiles::from("static"))
        .mount("/api", routes![routes::data])
}
