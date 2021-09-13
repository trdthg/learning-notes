use tokio::io::{AsyncWriteExt, Result};
use tokio::{
    fs::{read_to_string, File},
    io::AsyncReadExt,
    net::TcpStream,
};

const CRLF: &str = "\r\n";

pub async fn handle_request(mut stream: TcpStream) {
    let mut buf = [0; 4096];
    //
    stream.read(&mut buf).await;

    let write = |(content, status)| write_to_stream(stream, content, status);

    if matched(&buf, "/index") {
        println!("/index");
        write(handle_index().await).await
    } else {
        println!("/404");
        write(handle_404().await).await
    }
}

pub fn matched(buf: &[u8; 4096], route: &str) -> bool {
    let s = format!("GET {} HTTP/1.1{}", route, CRLF);
    println!("{}", String::from_utf8_lossy(buf));
    buf.starts_with(s.as_bytes())
}

pub async fn handle_index() -> (String, String) {
    (read_html("assets/index.html").await, get_status(200, "OK"))
}
pub async fn handle_404() -> (String, String) {
    (String::from("404 not found"), get_status(404, "NOT FOUND"))
}

pub async fn read_html(file_name: &str) -> String {
    read_to_string(file_name).await.unwrap()
}

pub fn get_status(code: i32, msg: &str) -> String {
    format!("HTTP/1.1 {} {}{}", code, msg, CRLF)
}

pub async fn write_to_stream(mut stream: TcpStream, content: String, status: String) {
    let content_type = format!("Content-Type: text/html;charset=utf-8{}", CRLF);
    let server = format!("Server: Rust{}", CRLF);
    let content_length = format!("Content-Length: {}{}", content.as_bytes().len(), CRLF);
    let response = format!(
        "{0}{1}{2}{3}{4}{5}",
        status, content_type, server, content_length, CRLF, content
    );
    stream.write(response.as_bytes()).await.unwrap();
    stream.flush();
}
