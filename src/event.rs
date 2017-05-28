extern crate nats;
use std::io::{self, Write, stdout};
use std::process::Command;
use nats::*;
use crossbeam::sync::SegQueue;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;

pub fn listen(rx: Receiver<String>) {

    loop {
        println!("started");
        let s = rx.recv();
        match s {
            Ok(v) => println!("{msg}", msg=v.to_string()),
            Err(r) => println!("{msg}", msg=r.to_string()),
        }
    }
    //
    // for _ in client.events() {
    //     // // execute { "platform": "linux", "repo": "ssh://xyz.git", "init": "init.sh" }
    //     // let output = Command::new("docker")
    //     //         .args(&["run", "-d", "moon",
    //     //             "git", "clone", "https://github.com/wayofthepie/emu-mos-6502",
    //     //             "&&", "echo 'test'"
    //     //             ])
    //     //         .spawn()
    //     //         .expect("ls command failed to start");
    //
    //     stdout().flush().ok().expect("Could not flush stdout");
    // }
}
