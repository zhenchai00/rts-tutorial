use std::{sync::mpsc::channel, thread, time::Duration};

enum Status{
    Good,
    Bad
}

struct Bun {
    status: Status
}

fn main() {
    let (tx, rx) = channel();

    let baker_thread = thread::spawn(move || {
        loop {
            for i in 1..12 {
                let mut bun = Bun { status: Status::Good };
                if i == 5 {
                    bun.status = Status::Bad;
                }
                tx.send(bun).expect("unable to send bun to the worker");
            }
            thread::sleep(Duration::from_secs(5));
        }
    });

    let worker_thread = thread::spawn(move || {
        let mut shelf = vec![];
        loop {
            let bun = rx.recv().unwrap();
            match bun.status {
                Status::Good => {
                    println!("Bun is good");
                    shelf.push(bun);
                }
                Status::Bad => {
                    println!("Bun is bad, throwing it away");
                }
                
            }
            println!("Shelf count: {}", shelf.len());
            thread::sleep(Duration::from_secs(1));
        }
    });

    baker_thread.join().unwrap();
    worker_thread.join().unwrap();
}