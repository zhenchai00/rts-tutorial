use std::{
    time::Duration,
    sync::Arc,
    sync::Mutex,
    ops::Deref,
};
use scheduled_thread_pool::ScheduledThreadPool;
use rand::Rng;

struct Stock {
    name: String,
    value: i32,
}
impl Deref for Stock {
    type Target = i32;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

pub fn stock_example() {
    // 1. create 5 stocks
    // 2. add stocks to a mutable vector
    // 3. wrap our vector in a arc / mutex
    let stocks = Arc::new(
        Mutex::new(
            vec![
                Stock {
                    name: "AAPL".to_string(),
                    value: 100,
                },
                Stock {
                    name: "GOOGL".to_string(),
                    value: 200,
                },
                Stock {
                    name: "AMZN".to_string(),
                    value: 30,
                },
                Stock {
                    name: "MSFT".to_string(),
                    value: 50,
                },
                Stock {
                    name: "TSLA".to_string(),
                    value: 500,
                },
            ]
        )
    );

    // 5 tasks/threads, each task will randomly select 
    // a stock every second, and increase it
    // send the updated value to the listener(main)
    // to print out the new value

    let pool = ScheduledThreadPool::new(5);
    let mut count = 1;
    for _ in 1..=5 {
        let stocks = stocks.clone();
        pool.execute_at_fixed_rate(
            Duration::from_secs(0),
            Duration::from_secs(1),
            move || {
                // attempt a lock
                let mut stocks = stocks.lock().unwrap();

                // select a stock
                let index = rand::thread_rng().gen_range(0..stocks.len());

                // increment stock
                stocks[index].value += 1;

                // send the value to the main
                println!("{}. stocks name {}: {}", count, stocks[index].name, stocks[index].value);
                count += 1;
            }
        );
    }
    loop{};
}