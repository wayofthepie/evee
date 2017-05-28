extern crate nats;
use std::io::{self, Write, stdout};
use std::process::Command;
use nats::*;
use crossbeam::sync::SegQueue;
use std::sync::mpsc::{Receiver};
use std::sync::{Mutex, Arc};

pub fn listen(rx: Receiver<String>, ref_queue: Arc<SegQueue<String>>) {

    loop {
        println!("started");
        let s = rx.recv();

        match s {
            Ok(v) => println!("{msg}", msg=v.to_string()),
            Err(r) => println!("{msg}", msg=r.to_string()),
        }

        let msg = ref_queue.try_pop().unwrap();
        println!("queue {msg}", msg=msg);
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
