use amiquip::{Connection, ConsumerMessage, ConsumerOptions, Exchange, Publish, QueueDeclareOptions, Result};

fn main() {
    let mut connection = Connection::insecure_open("amqp://guest:guest@localhost:5672").unwrap();
    let channel = connection.open_channel(None).unwrap();
    let queue = channel.queue_declare("drone", QueueDeclareOptions::default()).unwrap();
    let consumer = queue.consume(ConsumerOptions::default()).unwrap();
    println!("Waiting drone operator sending instructions. Press Ctrl-C to exit.");

    for (i, message) in consumer.receiver().iter().enumerate() {
        match message {
            ConsumerMessage::Delivery(delivery) => {
                let body = String::from_utf8_lossy(&delivery.body);
                println!("({:>3}) Drone received instruction = moving {}", i, body);
                consumer.ack(delivery).unwrap();
            }
            other => {
                println!("Consumer ended: {:?}", other);
                break;
            }
        }
    }
}