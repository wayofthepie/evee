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
mod execution;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

fn main() {
    let mut executions: HashMap<Uuid, execution::Execution> = HashMap::new();
    let execs_ref = Arc::new(Mutex::new(executions));
    rocket::ignite()
        .mount("/", routes![execution::execution])
        .launch();
}
