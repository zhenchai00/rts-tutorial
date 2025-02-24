use std::time::Duration;

use rand::random_range;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    let rand = random_range(1..5);
    let task_1 = async {
        sleep(Duration::from_secs(rand)).await;
        println!("Task - 1");
    };

    let task_2 = async {
        sleep(Duration::from_secs(rand)).await;
        println!("Task - 2");
    };

    let task_3 = async {
        sleep(Duration::from_secs(rand)).await;
        println!("Task - 3");
    };

    let task_4 = async {
        sleep(Duration::from_secs(rand)).await;
        println!("Task - 4");
    };

    let task_5 = async {
        sleep(Duration::from_secs(rand)).await;
        println!("Task - 5");
    };

    let task_6 = async {
        sleep(Duration::from_secs(rand)).await;
        println!("Task - 6");
    };

    let task_7 = async {
        sleep(Duration::from_secs(rand)).await;
        println!("Task - 7");
    };

    let task_8 = async {
        sleep(Duration::from_secs(rand)).await;
        println!("Task - 8");
    };

    tokio::select! {
        _ = task_1 => {},
        _ = task_2 => {},
        _ = task_3 => {},
        _ = task_4 => {},
        _ = task_5 => {},
        _ = task_6 => {},
        _ = task_7 => {},
        _ = task_8 => {},
    }
}