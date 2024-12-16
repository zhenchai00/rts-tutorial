use bma_benchmark::benchmark;
use std::hint::black_box;
use std::sync::{Mutex, RwLock};

pub fn mutex_test() {
    let value = 100;
    let mutex = Mutex::new(value);
    benchmark!(10000000, {
        // benchmarking the particular code block
        let _lock = mutex.lock().unwrap(); // acquire the lock
    });
}

pub fn rwlock_test() {
    let value = 100;
    let rwlock = RwLock::new(value);
    benchmark!(10000000, {
        // benchmarking the particular code block
        let _lock = rwlock.read().unwrap(); // acquire the read lock
    });
}
