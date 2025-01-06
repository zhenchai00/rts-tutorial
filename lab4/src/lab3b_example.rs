use std::{
    thread,
    sync::Arc, 
    sync::Mutex
};

pub fn exercise4 () {
    // atomic reference counter ARC
    let counter = Arc::new(Mutex::new(5));
    let mut handles = vec![];

    for i in 1..=10 {
        let counter = counter.clone();
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
            println!("\t Thread {i}: num: {num}");
        });
        handles.push(handle);
    } 
    for handle in handles {
        handle.join().unwrap();
    }
    println!("Result: {}", *counter.lock().unwrap());
}