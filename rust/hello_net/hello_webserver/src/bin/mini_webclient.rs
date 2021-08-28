use std::io::{self, ErrorKind, Read, Write};
use std::net::TcpStream;
use std::str;
use std::sync::mpsc::{self, TryRecvError};
use std::thread;
use std::time::Duration;

fn main() {
    let addr = ("127.0.0.1", 7878);
    let mut client = TcpStream::connect(addr).unwrap();

    let message: String = String::from("this is a message");
    client.write(&message.clone().into_bytes()).unwrap();
    client.write(&message.into_bytes()).unwrap();

}