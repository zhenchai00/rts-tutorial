use std::time::Duration;

use tokio::time::sleep;

#[tokio::main]
async fn main() {
    let low_priority = async {
        sleep(Duration::from_secs(3)).await;
        println!("longer task is considered a Low-priority task");
    };

    let high_priority = async {
        sleep(Duration::from_secs(1)).await;
        println!("shorter task is considered a High-priority task");
    };

    tokio::select! {
        _ = low_priority => {},
        _ = high_priority => {},
    }
}