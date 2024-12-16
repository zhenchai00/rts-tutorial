// use std::thread;
// use std::time::Duration;

use std::{thread, time::Duration};

fn main() {
    // // let mut min  = 0;
    // // let mut sec = 0;
    // let min  = 0;
    // let sec = 0;
    // let _t1 = thread::spawn(move ||{           // "||" is anonymous function 
    //     loop {
    //         print!("{}", min);
    //         // sec += 1;
    //         // if sec == 60 {
    //         //     sec = 0;
    //         //     min += 1;
    //         // }
    //         thread::sleep(Duration::from_secs(1));
    //     }
    // }).join().unwrap(); // closure

    // let _t2 = thread::spawn(move || {
    //     loop {
    //         println!(":{}", sec);
    //         // if sec == 60 {
    //         //     sec = 0;
    //         // }
    //         // sec += 1;
    //         thread::sleep(Duration::from_secs(1));
    //     }
    // }).join();


    // creating the timer with 2 threads running, 
    // which minutes is one thread and seconds is another thread
    // seconds
    let _ = thread::spawn(|| {
        let mut s = 0;
        loop{
            println!(":{}", s);
            s += 1;
            if s == 60 {
                s = 0;
            }
            thread::sleep(Duration::from_secs(1));
        }
    });

    // minutes
    let mut m = 0;
    let mut counter = 0;
    loop{
        print!("{}", m);
        counter += 1;
        if counter == 60 {
            m += 1;
            counter = 0;
        }
        thread::sleep(Duration::from_secs(1));
    }
}

// fn hello() {
//     println!("Hello");
// }

// fn get() -> i32 {
//     let mut x = 20;
//     x += 10;
//     println!("The value of x is: {}", x);

//     let x: i32 = 43;
//     println!("The value of x is: {}", x);

//     let x = get();
//     println!("The value of x is: {}", x);
//     return 42;
// }