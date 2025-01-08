use std::sync::mpsc;
use std::thread;

pub fn safety2 () {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        tx.send("hello from thread").unwrap();
    });
    let msg = rx.recv().unwrap();
    println!("received: {msg}");
}