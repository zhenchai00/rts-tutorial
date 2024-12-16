use std::{thread, time::Duration};

pub fn sw(){

    //seconds thread
    thread::spawn(||{
        let mut s = 0;
        loop{
            println!("{s}");
            s+=1;
            if s == 60{
                s = 0;
            }
            thread::sleep(Duration::from_secs(1));
        }
    });
    // minutes in the main thread
    let mut m = 0;
    let mut counter = 0;
    loop{
        print!("{m}:");
        counter +=1;
        if counter == 60{
            m +=1;
            counter = 0;
        }
        thread::sleep(Duration::from_secs(1));
    }
}