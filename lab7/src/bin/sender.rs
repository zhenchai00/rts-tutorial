use amiquip::{Connection, Exchange, Publish, Result};

fn main() -> Result<()> {
    // open connection
    let mut connection = Connection::insecure_open("amqp://guest:guest@localhost:5672")?;

    // open a channel - none says let the library choose the channel id
    let channel = connection.open_channel(None)?;

    // get a handle to the direct exchange on our channel
    let exchange = Exchange::direct(&channel);

    // publish a message to the "hello" queue
    exchange.publish(Publish::new("hey".as_bytes(), "hello"))?;

    let _ = connection.close();
    Ok(())
}