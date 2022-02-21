use std::fs::File;
use std::io::prelude::*;
use std::io::{copy, BufReader, Error, ErrorKind, Result};
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

enum Message {
    Connected(TcpStream),
    Quit,
}

fn main() -> Result<()> {
    let (host, port) = ("127.0.0.1", 7878);
    let tcplistener = TcpListener::bind((host, port))?;

    // 多个sender，单个receiver
    let (sender, receiver) = mpsc::channel();

    let sender_stream = sender.clone();
    // 开启新线程用来接受信息
    let accept_loop = thread::spawn(move || {
        while let Ok((stream, addr)) = tcplistener.accept() {
            // 受到的话就send给handler
            sender_stream.send(Message::Connected(stream));
        }
    });

    // 主线程用来处理连接，如果发来的信息是退出，就break
    while let Ok(message) = receiver.recv() {
        match message {
            Message::Connected(stream) => {
                let sender_quit = sender.clone();
                // 开启新线程处理信息
                thread::spawn(move || {
                    // 如果handler检测到退出请求，就send一个退出信号
                    if let Ok(HandleResult::Quit) = handle_connection(stream) {
                        sender_quit.send(Message::Quit).unwrap();
                    }
                });
            }
            Message::Quit => {
                break;
            }
        }
    }

    // accept_loop.join();
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
        Some(pos) => (&path[0..pos], &path[(pos + 1)..]),
        None => (path, ""),
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
                write!(
                    stream,
                    "HTTP/1.1 404 NOT FOUND\r\n\r\n<html><body>Not Found {}</body></html>",
                    path
                )?;
            }
        }
    }
    stream.flush()?;

    if query == "quit" {
        return Ok(HandleResult::Quit);
    }
    Ok(HandleResult::Ok)
}
