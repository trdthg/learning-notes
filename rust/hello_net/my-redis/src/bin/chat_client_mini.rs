use std::io::{self, ErrorKind, Read, Write};
use std::net::TcpStream;
use std::str;
use std::sync::mpsc::{self, TryRecvError};
use std::thread;
use std::time::Duration;

fn main() {
    const MESSAGE_SIZE: usize = 32;

    let addr = "127.0.0.1:9999";
    let mut client: TcpStream = TcpStream::connect(addr).unwrap();
    client
        .set_nonblocking(true)
        .expect("Failed to initiate non-blocking");
    let (sender, receiver) = mpsc::channel::<String>();
    // 从server接受新消息
    std::thread::spawn(move || loop {
        let mut buffer: Vec<u8> = vec![0; MESSAGE_SIZE];
        match client.read_exact(&mut buffer) {
            Ok(_) => {
                let message = buffer.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
                let message = String::from_utf8(message);
                println!("message: {:?}", message);
            }
            Err(_) => {

            }
        }
        match receiver.try_recv() {
            Ok(message) => {
                println!("正在发送");
                let mut buffer = message.clone().into_bytes();
                // Resize our buffer by our message size
                buffer.resize(MESSAGE_SIZE, 0);
                // Write all of our buffers into our client
                client.write_all(&buffer).expect("Writing to socket failed");
                client.write_all(&message.into_bytes()).unwrap();
            },
            Err(e) => {

            }
        }
    });

    // 从slient接受输入, 并向server发送新消息
    loop {
        let mut buffer = String::new();
        std::io::stdin()
            .read_line(&mut buffer)
            .expect("Reading from stdin failed");
        let message = buffer.trim().to_string();
        println!("输入了: {}", message);
        if message == "exit" {
            println!("*********************************");
            println!("*********** GOOD BYE ************");
            println!("*********************************");
            break;
        } else {
            sender.send(buffer).unwrap();
            // sender.send(buffer)
        }
    }

}