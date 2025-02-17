use std::{
    thread,
    time::Duration,
    sync::mpsc,
};

fn main () {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let mut min_count = 0;
        loop {
            tx.send(min_count).unwrap();
            min_count += 1;
            thread::sleep(Duration::from_secs(60));
        }
    });

    loop {
        match rx.try_recv() {
            Ok(min_count) => {
                println!("Minute count: {}", min_count);
            },
            Err(e) => {
                println!("Error: {:?}", e);
            }
        }
        thread::sleep(Duration::from_secs(1));
    }
}