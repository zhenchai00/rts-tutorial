use get_user_input::set_from_input;
use amiquip::{Connection, Exchange, Publish, Result};

fn main() -> Result<()> {
    loop{
        // open connection
        let mut connection = Connection::insecure_open("amqp://guest:guest@localhost:5672")?;

        // open a channel - none says let the library choose the channel id
        let channel = connection.open_channel(None)?;

        // get a handle to the direct exchange on our channel
        let exchange = Exchange::direct(&channel);

        // controller to send message to drone
        let mut s = String::new();
        set_from_input!(&mut s, "Enter direction to send to drone (forward, back, left, right): ");

        // publish a message to the "hello" queue
        exchange.publish(Publish::new(s.as_bytes(), "drone"))?;

        let _ = connection.close();
    }
}