#![feature(proc_macro_hygiene)]
#![feature(decl_macro)]

#[macro_use]
extern crate rocket;

mod routes;
use routes::*;

fn main() {
    rocket::ignite()
        .mount("/", routes![index, files, submit_task])
        .launch();
}
