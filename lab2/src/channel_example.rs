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
        thread::sleep(Duration::from_secs(1));
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
        thread::sleep(Duration::from_secs(1));
        tx.send(data).unwrap();
    });

    // receive and print the data
    println!("crossbeam_channel_ex");
    println!("main thread is waiting for data...");
    let data = rx.recv().unwrap();
    println!("value received: {data}");
}

pub fn assessment_channel() {
    let (tx, rx) = channel();
    let sec_thread = thread::spawn(move || {
        let mut sec_count = 0;
        loop {
            sec_count += 1;
            thread::sleep(Duration::from_secs(1));
            tx.send(sec_count).unwrap();
            if sec_count == 60 {
                sec_count = 0;
            }
        };
    });
    let min_thread = thread::spawn(move || {
        let mut min_count = 0;
        loop {
            let mut sec_count = rx.recv().unwrap();
            if sec_count == 60 {
                min_count += 1;
                sec_count = 0;
            }
            println!("{min_count}:{number:0>2}", number = sec_count);
        };
    });

    sec_thread.join().unwrap();
    min_thread.join().unwrap();
}