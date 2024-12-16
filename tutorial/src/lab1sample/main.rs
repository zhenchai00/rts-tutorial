mod stopwatch;

use std::thread;
use std::time::Duration;

use stopwatch::sw;
fn main() {
    sw();
    //topwatch::sw();
    // //thread::spawn(logic);
    // thread::spawn(||{
    //     println!("hello for the other thread");
    // });
    // loop{
    //     println!("Bye from the main thread");
    //     thread::sleep(Duration::from_secs(1));
    // }
    
    //||{} closure
}

