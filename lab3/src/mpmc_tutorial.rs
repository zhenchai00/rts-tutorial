use std::{
    thread,
    time::Duration
};
use crossbeam_channel::unbounded;

pub fn actual_stopwatch_three() {
    let (sec_tx, sec_rx) = unbounded();
    let (min_tx, min_rx) = unbounded();
    let sec_rx2 = sec_rx.clone();

    // seconds thread
    thread::spawn(move || {
        let mut sec_count = 0; // integer implement traits, make it copyable, can move to another scope
        loop {
            sec_count += 1;
            sec_tx.send(sec_count).unwrap();
            sec_tx.send(sec_count).unwrap();
            if sec_count == 60 {
                sec_count = 0;
            }
            thread::sleep(Duration::from_secs(1));
        };
    });

    // minutes thread
    thread::spawn(move || {
        let mut min_count = 0;
        loop {
            if sec_rx.recv().unwrap() == 60 {
                min_count += 1;
            }
            min_tx.send(min_count).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // main thread display
    loop {
        let sec_count = sec_rx2.recv().unwrap();
        let min_count = min_rx.recv().unwrap();
        println!("{min}:{sec:0>2}", min = min_count, sec = sec_count);
    }
}