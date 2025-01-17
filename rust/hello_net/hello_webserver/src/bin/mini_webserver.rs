extern crate serde_json;

use std::collections::HashMap;
use std::fs;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

use hello_webserver::ThreadPool;

use serde_json::Value;

// req
// Method Request-URI HTTP-Version CRLF
// headers CRLF
// message-body
// res
// HTTP-Version Status-Code Reason-Phrase CRLF
// headers CRLF
// message-body
fn main() {
    let (host, port) = ("127.0.0.1", 7878);
    let listener = TcpListener::bind((host, port)).unwrap();

    let pool = ThreadPool::new(4);
    println!("----------server started in {}:{}----------", host, port);
    // take(2) 能限制接受的请求数
    for stream in listener.incoming().take(20) {
        println!("++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++");
        let stream = stream.unwrap();
        pool.spawn(|| {
            handle_connection(stream);
        })
    }
}

#[derive(serde::Deserialize, Debug)]
struct Test {
    body1: i32,
    body2: String,
}
fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    let display = String::from_utf8_lossy(&buffer[..]);
    let display = display.splitn(2, "\u{0}").collect::<Vec<&str>>()[0];
    println!("{:#?}", display);
    println!("{}", display);
    let display_vec: Vec<&str> = display.split("\r\n\r\n").collect();
    let head: Vec<&str> = display_vec[0].split("\r\n").collect();

    // header
    let mut headers: HashMap<String, String> = HashMap::new();
    for header in head[1..].to_vec().iter() {
        let header: Vec<&str> = header.splitn(2, ": ").collect();
        headers.insert(header[0].to_string(), header[1].to_string());
    }

    // method, url
    let head_vec: Vec<&str> = head[0].split(' ').collect();
    let (method, url) = (head_vec[0], head_vec[1]);
    let url_vec: Vec<&str> = url.split('?').collect();
    let url = url_vec[0];

    let mut args: Option<HashMap<String, String>> = None;
    if url_vec.len() > 1 && url_vec[1] != "" {
        args = Some(str_to_hashmap(url_vec[1]));
    }
    let mut body: Option<serde_json::Value> = None;
    let mut form: Option<HashMap<String, String>> = None;
    if let Some(content_type) = headers.get("Content-Type") {
        if content_type == r"application/x-www-form-urlencoded" {
            println!(r"--- application/x-www-form-urlencoded ---");
            let info: &str = display_vec[1];
            form = Some(str_to_hashmap(info));
        } else if content_type == "application/json" {
            println!(r"--- application/json ---");
            let info: &str = display_vec[1];
            body = match serde_json::from_str::<Value>(&info) {
                Ok(map) => Some(map),
                Err(err) => None,
            };
            // body = match serde_json::from_str::<Test>(&info){
            //     Ok(map) => Some(map),
            //     Err(err) => None
            // };
        } else if content_type.starts_with(r"multipart\form-data") {
            // 涉及到图片上传, 缓冲区大小一定要足够才行
            println!(r"--- multipart\form-data ---");
            let info: Vec<&str> = display_vec[1..].to_vec();
            println!("{:?}", info);
            for row in info {
                let infos: Vec<&str> = row.split("\r\n").collect();
                let entrys = infos[2];
            }
        }
    }
    println!("1. {:?}\n2. {:?}\n3. {:?}\n", args, form, body);

    // println!("{}\n{}\n{:?}\n{:?}", method, url, args);

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n", "hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n", "hello.html")
    } else {
        // ("HTTP/1.1 404 NOT FOUND\r\nContent:sssss\r\n{}\r\n\r\n", "404.html", body)
        let body = r#"Test { body1: 1, body2: "sssss".to_string()}"#;
        ("HTTP/1.1 404 NOT FOUND\r\n", "404.html")
    };
    let headers = r#"
        {
            Content-Length: 34
        }
    "#;
    let contents = fs::read_to_string(filename).unwrap();
    let response = format!("{}{}{}", status_line, headers, contents);
    stream.write(response.as_bytes()).unwrap();
    // flush 会等待并阻塞程序执行直到所有字节都被写入连接中；TcpStream 包含一个内部缓冲区来最小化对底层操作系统的调用。
    stream.flush().unwrap();
}

fn str_to_hashmap(string: &str) -> HashMap<String, String> {
    let params: Vec<&str> = string.split('&').collect();
    let mut args: HashMap<String, String> = HashMap::new();
    // args
    for param in params.iter() {
        let a: Vec<&str> = param.split('=').collect();
        args.insert(a[0].to_string(), a[1].to_string());
    }
    args
}
