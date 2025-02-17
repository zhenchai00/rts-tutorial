use std::{sync::mpsc, thread, time::Duration};

fn main() {
    let (sender, receiver) = mpsc::channel();

    thread::spawn(move || {
        thread::sleep(Duration::from_secs(2));
        sender.send("hello").unwrap();
    });

    loop {
        // match receiver.try_recv() {
        //     Ok(value) => {
        //         println!("Received: {}", value);
        //         break;
        //     }
        //     Err(_) => {
        //         println!("Not yet");
        //     }
        // }
        receiver.try_recv().unwrap();
        thread::sleep(Duration::from_secs(1));
    }
}