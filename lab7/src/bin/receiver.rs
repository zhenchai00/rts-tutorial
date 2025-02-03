use amiquip::{Connection, ConsumerMessage, ConsumerOptions, QueueDeclareOptions};

fn main() {
    let mut msg = "";

    // open connection
    let mut connection = Connection::insecure_open("amqp://guest:guest@localhost:5672").unwrap();

    // open a channel - none says let the library choose the channel id
    let channel = connection.open_channel(None).unwrap();

    // declare the "hello" queue
    let queue = channel.queue_declare("hello", QueueDeclareOptions::default()).unwrap();

    // start a consumer
    let consumer = queue.consume(ConsumerOptions::default()).unwrap();
    println!("Waiting for messages. Press Ctrl-C to exit.");

    for (i, message) in consumer.receiver().iter().enumerate() {
        match message {
            ConsumerMessage::Delivery(delivery) => {
                let body = String::from_utf8_lossy(&delivery.body);
                println!("({:>3}) Received [{}]", i, body);
                msg = &body;
                consumer.ack(delivery).unwrap();
            }
            other => {
                println!("Consumer ended: {:?}", other);
                break;
            }
        }
    }    
    let _ = connection.close();
}