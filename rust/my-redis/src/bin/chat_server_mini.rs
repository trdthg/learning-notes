use std::net::{ TcpListener, TcpStream };
use std::io::{ErrorKind, Read, Write};
use std::sync::mpsc;

fn main() {
    const MESSAGE_SIZE: usize = 32;
    let addr = "127.0.0.1:9999";
    let listener = TcpListener::bind(addr).unwrap();
    listener
        .set_nonblocking(true)
        .expect("Failed to initialize non-blocking");
    let mut clients = Vec::new();

    let (sender, receiver) = mpsc::channel::<String>();
    // 尝试获取连接
    loop {
        if let Ok((mut socket, addr)) = listener.accept() {
            let sender2 = sender.clone();
            println!("Client {:?}: connected", addr);
            clients.push(socket.try_clone().unwrap());
            // 接受客户端发来的消息
            println!("{}", "waiting");
            std::thread::spawn(move || {
                loop {
                    let mut buffer: Vec<u8> = vec![0; MESSAGE_SIZE];
                    match socket.read_exact(&mut buffer) {
                        Ok(_) => {
                            let message = buffer.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
                            let message = String::from_utf8(message).unwrap();
                            println!("{}: {}", addr, message);
                            sender2.send(message).unwrap();
                        },
                        Err(e) => {

                        }
                    }
                }
            });
        }
        if let Ok(message) = receiver.try_recv() {
            for mut client in clients.iter() {
                let mut buffer = message.clone().into_bytes();
                buffer.resize(MESSAGE_SIZE, 0);
                client.write_all(&buffer).unwrap();
            }
            clients = clients.into_iter().filter(|mut client| {
                let mut buffer = message.clone().into_bytes();
                buffer.resize(MESSAGE_SIZE, 0);
                client.write_all(&buffer).is_ok()
            }).collect::<Vec<_>>();

            clients = clients.into_iter().filter_map(|mut client| {
                let mut buffer = message.clone().into_bytes();
                buffer.resize(MESSAGE_SIZE, 0);
                client.write_all(&buffer).map(|_| client).ok()
            }).collect::<Vec<_>>();

        }

    }

}