use std::{
    sync::mpsc::channel,
    sync::RwLock,
    sync::Mutex,
    thread, 
};

use crossbeam::channel::unbounded;

pub fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

pub fn fibonacci_iteration (n: u64) -> u64 {
    let mut prev = 0;
    let mut curr = 1;
    for _ in 1..n {
        let tmp = prev;
        prev = curr;
        curr = tmp + curr;
    }
    curr
}

pub fn mpsc_test () {
    let (tx, rx) = channel();
    thread::spawn(move || {
        tx.send(42).unwrap();
    });
    rx.recv().unwrap();
}

pub fn crossbeam_mpsc_test () {
    let (tx, rx) = unbounded();
    thread::spawn(move || {
        tx.send(42).unwrap();
    });
    rx.recv().unwrap();
}

pub fn rw_lock_test () {
    let lock = RwLock::new(0);
    thread::spawn(move || {
        let mut data = lock.write().unwrap();
        *data = 42;
    }).join().unwrap();
}

pub fn mutex_test () {
    let lock = Mutex::new(0);
    thread::spawn(move || {
        let mut data = lock.lock().unwrap();
        *data = 42;
    }).join().unwrap();
}

pub fn vec_test () {
    let mut v = Vec::new();
    for i in 0..1000 {
        v.push(i);
    }
}

pub fn vec_macro_test () {
    let _ = (0..1000).collect::<Vec<_>>();
}