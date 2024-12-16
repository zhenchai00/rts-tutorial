use std::{
    thread,
    time::Duration,
    sync::mpsc::channel
};

pub fn actual_stopwatch() {
    let (sec_tx, sec_rx) = channel();
    let (min_tx, min_rx) = channel();

    // seconds thread
    thread::spawn(move || {
        let mut sec_count = 0; // integer implement traits, make it copyable, can move to another scope
        loop {
            sec_tx.send(sec_count).unwrap();
            sec_count += 1;
            if sec_count == 60 {
                sec_count = 0;
            }
            thread::sleep(Duration::from_secs(1));
        };
    });

    // minutes thread
    thread::spawn(move || {
        let mut min_count = 0; // integer implement traits, make it copyable, can move to another scope
        loop {
            min_tx.send(min_count).unwrap();
            min_count += 1;
            thread::sleep(Duration::from_secs(60));
        }
    });

    // main thread display
    let mut current_min = 0;
    loop {
        let sec_count = sec_rx.recv().unwrap();
        match min_rx.try_recv() {
            Ok(value) => {
                current_min = value;
            }
            Err(_) => {}
        }
        println!("{min}:{sec:0>2}", min = current_min, sec = sec_count);
    }
}

pub fn actual_stopwatch_two() {
    let (sec_tx, sec_rx) = channel();
    let (sec_min_tx, sec_min_rx) = channel();
    let (min_tx, min_rx) = channel();

    // seconds thread
    thread::spawn(move || {
        let mut sec_count = 0; // integer implement traits, make it copyable, can move to another scope
        loop {
            sec_tx.send(sec_count).unwrap();
            sec_count += 1;
            if sec_count == 60 {
                sec_min_tx.send(sec_count).unwrap();
                sec_count = 0;
            }
            thread::sleep(Duration::from_secs(1));
        };
    });

    // minutes thread
    thread::spawn(move || {
        let mut min_count = 0; // integer implement traits, make it copyable, can move to another scope
        loop {
            match sec_min_rx.try_recv() {
                Ok(_) => {
                    min_count += 1;
                }
                Err(_) => {}
            }
                min_tx.send(min_count).unwrap();
                thread::sleep(Duration::from_secs(1));
        }
    });

    // main thread display
    loop {
        let sec_count = sec_rx.recv().unwrap();
        let min_count = min_rx.recv().unwrap();
        println!("{min}:{sec:0>2}", min = min_count, sec = sec_count);
    }
}

pub fn enhanced_stopwatch() {
    let (tx, rx) = channel();
    let sec_thread = thread::spawn(move || {
        let mut sec_count = 0;
        loop {
            tx.send(sec_count).unwrap();
            sec_count += 1;
            if sec_count == 60 {
                sec_count = 0;
            }
            thread::sleep(Duration::from_secs(1));
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