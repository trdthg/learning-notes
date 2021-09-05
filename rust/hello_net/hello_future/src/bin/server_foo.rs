use std::{io::{Read, Result, Write}, net::{TcpListener, TcpStream}};
use std::time::Duration;
use std::thread;

fn handle_connection(mut stream: TcpStream, wait_time: u64) -> Result<()> {
    let mut buf = [0; 1024];
    // 服务端需要接受用户输入, 但是为了而防止恶意输入, 不能无限等待用户, 需要设置一定的缓冲区
    // 客户端可以通过\n表示返回信息结束
    loop {
        let buf_read = stream.read(&mut buf)?;
        if buf_read == 0 {
            return Ok(())
        }
        println!("read 1 {} buf_now {}", buf_read, String::from_utf8_lossy(&buf));
        thread::sleep(Duration::from_secs(wait_time));
        stream.write("foo -------------".as_bytes())?;
        stream.write(&buf[..buf_read])?;
        stream.write("这是第二段请求反正".as_bytes())?;
        stream.write("\n".as_bytes())?;
    }
}

fn main() -> Result<()> {

    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    for stream in listener.incoming() {
        handle_connection(stream?, 1)?;
    }
    Ok(())
}