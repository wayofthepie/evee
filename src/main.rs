#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate nats;
extern crate rocket;
extern crate crossbeam;
mod event;
use rocket::State;
use std::sync::{Mutex, Arc};
use crossbeam::sync::SegQueue;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;

pub struct S (Sender<String>);

unsafe impl Sync for S {}
unsafe impl Send for S {}

#[get("/<name>/<age>")]
fn hello(name: String, age: u8, queue: State<Arc<SegQueue<String>>>, sender: State<S>) -> String {
    let x = sender.0.send(name.to_string()).unwrap();
    queue.push(age.to_string());
    format!("Hello, {} year old named {}!", age, name)
}

fn main() {
    let queue: SegQueue<String> = SegQueue::new();
    let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();
    let ref_queue = Arc::new(queue);
    crossbeam::scope(|scope| {
        let ref_queue_clone = ref_queue.clone();
        scope.spawn(||{
            event::listen(rx, ref_queue);
        });
        scope.spawn(||{
            rocket::ignite()
                .manage(ref_queue_clone)
                .manage(S(tx))
                .mount("/hello", routes![hello])
                .launch();
        });
    });
}
