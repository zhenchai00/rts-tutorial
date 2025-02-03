use std::{thread, time::Duration};
use amiquip::{Connection,Exchange,Publish,Result};
use rand::Rng;

fn generate_stock() -> (String, i32) {
    let stocks = vec!["AAPL", "GOOGL", "AMZN", "MSFT", "TSLA"];
    let stock_name = stocks[rand::rng().random_range(0..stocks.len())].to_string();
    let stock_value = rand::rng().random_range(100..1000);
    return (stock_name, stock_value)
}

fn main() -> Result<()> {
    let mut connection = Connection::insecure_open("amqp://guest:guest@localhost:5672")?;
    let channel = connection.open_channel(None)?;

    let exchange = Exchange::direct(&channel);

    loop {
        // Simulate stock generation
        let (stock_name, stock_value) = generate_stock();
        let message = format!("{}: ${:.2}", stock_name, stock_value);

        // Publish the stock data to the "stock_data" queue
        exchange.publish(Publish::new(message.as_bytes(), "stock_data"))?;

        println!("Generated and sent: {}", message);

        // Wait for a while before generating the next stock
        thread::sleep(Duration::from_secs(1));
    }
    connection.close();
    Ok(())
}