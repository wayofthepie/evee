#![feature(plugin)]
#![plugin(rocket_codegen)]
#![feature(plugin)]
#![cfg_attr(test, plugin(stainless))]
#[macro_use]
extern crate log;
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
    let executions: HashMap<Uuid, execution::Execution> = HashMap::new();
    let ref_execs = Arc::new(Mutex::new(executions));
    rocket::ignite()
        .mount("/", routes![
            execution::execution,
            execution::get_execution
            ])
        .manage(ref_execs)
        .launch();
}
