#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate spectral;
extern crate uuid;

mod event;

fn main() {
    rocket::ignite()
        .mount("/", routes![event::event])
        .launch();
}
