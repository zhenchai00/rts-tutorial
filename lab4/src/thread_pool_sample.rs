use std::{thread, time::Duration};
use threadpool::ThreadPool;

pub fn thread_pool_fn () {
    for _ in 1..=10 {
        thread::spawn(|| {
            println!("hello from the thread");
            thread::sleep(Duration::from_millis(500));
            println!("good bye from the thread");
        }).join().unwrap();
    }

    let pool = ThreadPool::new(3);
    
    for _ in 1..=10 {
        pool.execute(|| {
            println!("\t hello from thread pool");
            thread::sleep(Duration::from_millis(500));
            println!("\t good bye from the threadpool");
        });
    }
}