use std::{
    sync::mpsc::channel, // multiple producer, single consumer (MPSC)
    thread,
    time::Duration,
};
use crossbeam_channel::unbounded;

pub fn channel_ex() {
    let (tx, rx) = channel();

    thread::spawn(move || {
        // send data to the main thread
        let data = 101;
        thread::sleep(Duration::from_secs(5));
        tx.send(data).unwrap();
    });

    // receive and print the data
    println!("channel_ex");
    println!("main thread is waiting for data...");
    let data = rx.recv().unwrap();
    println!("value received: {data}");
}

pub fn crossbeam_channel_ex() {
    let (tx, rx) = unbounded();

    thread::spawn(move || {
        // send data to the main thread
        let data = 101;
        thread::sleep(Duration::from_secs(5));
        tx.send(data).unwrap();
    });

    // receive and print the data
    println!("crossbeam_channel_ex");
    println!("main thread is waiting for data...");
    let data = rx.recv().unwrap();
    println!("value received: {data}");
}
