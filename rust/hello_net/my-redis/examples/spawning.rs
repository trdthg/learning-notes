use std::io::Result;
use tokio::net::{TcpListener, TcpStream};
#[tokio::main]
pub async fn main() -> Result<()> {
    let listener = TcpListener::bind(("127.0.0.1", 6379)).await.unwrap();
    println!("started listener on 127.0.0.1:6379");
    loop {
        let (socket, addr) = listener.accept().await.unwrap();
        println!("{}", addr);
        // A new task is spawned for each inbound socket. The socket is moved to the new task and processed there.
        tokio::spawn(async move {
            handle_connection(socket).await;
        });
    }
}

async fn handle_connection(socket: TcpStream) {
    use mini_redis::{Command, Connection, Frame};
    use std::collections::HashMap;

    // A hashmap is used to store data
    let mut db = HashMap::new();

    // Connection, provided by `mini-redis`, handles parsing frames from the socket
    let mut connection = Connection::new(socket);

    // use `read_frame` to receive a command from the connection
    while let Some(frame) = connection.read_frame().await.unwrap() {
        println!("Got: {:?}", frame);
        let response = match Command::from_frame(frame).unwrap() {
            Command::Set(cmd) => {
                db.insert(cmd.key().to_string(), cmd.value().to_vec());
                Frame::Simple("OK".to_string())
            }
            Command::Get(cmd) => {
                if let Some(value) = db.get(cmd.key()) {
                    Frame::Bulk(value.clone().into())
                } else {
                    Frame::Null
                }
            }
            cmd => panic!("unimplemented {:?}", cmd),
        };

        // let response = Frame::Error("unimplemented".to_string());
        // write the response to the client
        connection.write_frame(&response).await.unwrap();
    }
}
