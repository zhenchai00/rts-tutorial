use get_user_input::set_from_input;
use amiquip::{Connection, Exchange, Publish, Result};

fn main(){
    loop{
        let mut msg = "".to_string();
        print!("Enter instructions ({}):", msg);

        set_from_input!(&mut msg);
    
        // get user input
        println!("CONTROLLER: sending instructions: {}",msg );
        let _ = send_msg(msg);
    }
}

fn send_msg(directions: String)-> Result<()> {
    // Open connection.
    let mut connection = Connection::insecure_open("amqp://guest:guest@localhost:5672")?;

    // Open a channel - None says let the library choose the channel ID.
    let channel = connection.open_channel(None)?;

    // Get a handle to the direct exchange on our channel.
    let exchange = Exchange::direct(&channel);

    // Publish a message to the "hello" queue.
    exchange.publish(Publish::new(directions.as_bytes(), "hello"))?;

    connection.close()
}