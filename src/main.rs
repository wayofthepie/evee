#![feature(plugin)]
#![plugin(rocket_codegen)]
#[macro_use]
extern crate lazy_static;
extern crate nats;
extern crate rocket;
extern crate crossbeam;
mod event;
use std::io::{self, Write, stdout};
use std::process::Command;
use nats::*;
use std::thread;
use rocket::State;
use rocket::request::{self, Request, FromRequest};
use rocket::Outcome::{Success, Failure};
use std::boxed::Box;
use std::collections::HashMap;
use std::sync::{Mutex, Arc};
use crossbeam::sync::SegQueue;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;

pub struct S (Sender<String>);
impl S {
    pub fn get(self) -> Sender<String> {
        return self.0;
    }
}

unsafe impl Sync for S {}
unsafe impl Send for S {}

#[get("/<name>/<age>")]
fn hello(name: String, age: u8, state: State<S>) -> String {
    let x = state.0.send(name.to_string()).unwrap();
    format!("Hello, {} year old named {}!", age, name)
}

fn main() {
    let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();
    thread::spawn(|| {
        event::listen(rx);
    });
    rocket::ignite().manage(S(tx)).mount("/hello", routes![hello]).launch();
}
