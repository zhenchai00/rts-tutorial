use rand::Rng;
use tokio::{
    sync::mpsc::channel,
    time::sleep,
    time::Duration,
};

fn generate_stock() -> (String, i32) {
    let stocks = vec!["AAPL", "GOOGL", "AMZN", "MSFT", "TSLA"];
    let stock_name = stocks[rand::rng().random_range(0..stocks.len())].to_string();
    let stock_value = rand::rng().random_range(100..1000);
    return (stock_name, stock_value)
}

#[tokio::main]
async fn main () {
    let (tx, mut rx) = channel(32);
    let tx1 = tx.clone();

    tokio::spawn(async move {
        loop {
            let (stock_name, stock_value) = generate_stock();
            tx.send((stock_name, stock_value)).await.unwrap();
            sleep(Duration::from_secs(1)).await;
        }
    });
    tokio::spawn(async move {
        loop {
            let (stock_name, stock_value) = generate_stock();
            tx1.send((stock_name, stock_value)).await.unwrap();
            sleep(Duration::from_secs(1)).await;
        }
    });

    while let Some((stock_name, stock_value)) = rx.recv().await {
        println!("Stock: {} Value: {}", stock_name, stock_value);
    }
}
