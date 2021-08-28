use std::io::{ Result, BufReader, Error, ErrorKind, copy };
use std::sync::mpsc;
use std::thread;
use std::net::{ TcpListener, TcpStream };
use std::time::Duration;
use std::fs::File;
use std::io::prelude::*;

enum Message {
    Connected(TcpStream),
    Quit,
}

fn main() -> Result<()> {

    let (host, port) = ("127.0.0.1", 7878);
    let tcplistener = TcpListener::bind((host, port))?;

    let (sender, receiver) = mpsc::channel();
    
    let sender_stream = sender.clone();
    thread::spawn(move || {
        while let Ok((stream, addr)) = tcplistener.accept() {
            sender_stream.send(Message::Connected(stream));
        }
    });

    while let Ok(message) = receiver.recv() {
        match message {
            Message::Connected(stream) => {
                let sender_quit = sender.clone();
                thread::spawn(move || {
                    if let Ok(HandleResult::Quit) = handle_connection(stream) {
                        sender_quit.send(Message::Quit).unwrap();
                    }
                });
            },
            Message::Quit => {
                break;
            }
        }
    }

    Ok(())
}

enum HandleResult {
    Ok,
    Quit,
}

fn handle_connection(mut stream: TcpStream) -> Result<HandleResult> {
    let mut stream_str = String::new();
    // read_line只能获取第一行
    BufReader::new(&stream).read_line(&mut stream_str)?;
    let stream_subs: Vec<&str> = stream_str.split(" ").collect();

    println!("#1: {}", stream_str);
    let stream_subs: Vec<&str> = stream_str.split(" ").collect();
    let (method, path) = (stream_subs[0], stream_subs[1]);

    let (path, query) = match path.find("?") {
        Some(pos) => (&path[0..pos], &path[(pos+1)..]),
        None => (path, "")
    };
    println!("#3{}, {}", path, query);

    if query == "sleep" {
        thread::sleep(Duration::from_secs(5));
    }
    if path == "/" {
        stream.write(b"HTTP/1.1 200 OK\r\n\r\n<html><body>Welcome</body></html>")?;
        // write!(stream, "HTTP/1.1 200 OK\r\n\r\n<html><body>Welcome</body></html>")?;
    } else {
        let relative_path = match path.strip_prefix("/") {
            Some(p) => p,
            None => path,
        };
        match File::open(relative_path) {
            Ok(mut f) => {
                write!(stream, "HTTP/1.1 200 OK\r\n\r\n")?;
                copy(&mut f, &mut stream)?;
            }
            Err(err) => {
                write!(stream, "HTTP/1.1 404 NOT FOUND\r\n\r\n<html><body>Not Found {}</body></html>", path)?;
            }
        }
    }
    stream.flush()?;

    if query == "quit" {
        return Ok(HandleResult::Quit)
    }
    Ok(HandleResult::Ok)
}