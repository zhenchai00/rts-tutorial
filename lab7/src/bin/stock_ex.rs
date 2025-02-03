use amiquip::{Connection, ConsumerMessage, ConsumerOptions, QueueDeclareOptions};

fn main() {
    let mut connection = Connection::insecure_open("amqp://guest:guest@localhost:5672").unwrap();
    let channel = connection.open_channel(None).unwrap();
    let queue = channel.queue_declare("stock_data", QueueDeclareOptions::default()).unwrap();
    let consumer = queue.consume(ConsumerOptions::default()).unwrap();
    println!("Main exchange listening for stock data. Press Ctrl-C to exit.");

    for (i, message) in consumer.receiver().iter().enumerate() {
        match message {
            ConsumerMessage::Delivery(delivery) => {
                let body = String::from_utf8_lossy(&delivery.body);
                println!("({:>3}) Stock exchange received stock = {}", i, body);
                consumer.ack(delivery).unwrap();
            }
            other => {
                println!("Consumer ended: {:?}", other);
                break;
            }
        }
    }
    connection.close().unwrap();
}