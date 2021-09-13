use std::io::{BufRead, BufReader, Read, Write};
use std::thread;
use std::time::Duration;
use std::{io::Result, net::TcpStream};

fn req(addr: &str, msg: &str) -> Result<()> {
    // 客户端需要接受到完整的信息, 可以用vec_buf
    let mut stream = TcpStream::connect(addr)?;
    // 向server分段发送请求
    // let write_buf = stream.write(msg.as_bytes())?;
    // println!("write 1 {} send_msg {}", write_buf, msg);
    let write_buf = stream.write("message2whichisverylongthatbigglethan10".as_bytes())?;
    // println!("write 2 {} send_msg {}", write_buf, msg);
    // 尝试合并接受请求

    // 使用vec读取, 没有大小限制
    let mut buf = Vec::new();
    let mut reader = BufReader::new(&stream);
    // reader.read_until(b'\n', &mut buf)?;
    // println!("{}", String::from_utf8_lossy(&buf));
    // buf.clear();
    reader.read_until(b'\n', &mut buf)?;
    println!("{}", String::from_utf8_lossy(&buf));

    // 使用buf读取, 有大小限制
    // let mut buf = [0; 10];
    // let read_buf = stream.read(&mut buf)?;
    // reader.read(&mut buf)?;
    // println!("{}", String::from_utf8_lossy(&buf));
    // println!("read 1: {}, buf_now {:?}", read_buf, String::from_utf8_lossy(&buf));
    // // 每次读入都只会从开头开始覆盖内容

    // let read_buf = stream.read(&mut buf)?;
    // println!("read 2: {}, buf_now {:?}", read_buf, String::from_utf8_lossy(&buf));
    Ok(())
}

async fn async_req(addr: &str, msg: &str) -> Result<()> {
    let mut stream = TcpStream::connect(addr)?;
    let write_buf = stream.write("message2whichisverylongthatbigglethan10".as_bytes())?;
    let mut buf = Vec::new();
    let mut reader = BufReader::new(&stream);
    reader.read_until(b'\n', &mut buf)?;
    println!("{}", String::from_utf8_lossy(&buf));
    Ok(())
}

async fn req1(addr: &str, msg: &str) -> usize {
    async_req(addr, msg).await;
    async_req(addr, msg).await;
    println!("1");
    1
}

async fn req2(addr: &str, msg: &str) -> usize {
    // async_req(addr, msg).await;
    println!("2");
    2
}
async fn req_both() -> (usize, usize) {
    let r2 = req2("127.0.0.1:8081", "bar");
    let r1 = req1("127.0.0.1:8080", "foo");
    futures::join!(r1, r2)
}

pub fn main() -> Result<()> {
    let start = std::time::Instant::now();
    // req("127.0.0.1:8080", "req1")?;
    // println!("{:?}", start.elapsed());
    // req("127.0.0.1:8081", "req1")?;
    // println!("{:?}", start.elapsed());

    // let f = req_both();
    // let c = futures::executor::block_on(f);
    // println!("{:?}", start.elapsed());
    use futures::try_join;

    // let a = async { let mut a = 0;for i in 0..1_000_000 { a += 1;}; println!("1"); Ok::<i32, i32>(1) };
    // let a2 = async { let mut a = 0;for i in 0..1_000_000 { a += 1;}; println!("1"); Ok::<i32, i32>(1) };
    // let b = async { println!("2"); Ok::<i32, i32>(2) };
    // let b2 = async { println!("2"); Ok::<i32, i32>(2) };
    // futures::executor::block_on(async {
    //     println!("{:?}", start.elapsed());
    //     println!("{:?}", try_join!(a, b));
    //     (a2.await, b2.await);
    //     println!("{:?}", start.elapsed());
    // });

    // futures::executor::block_on(async {
    //     use futures::try_join;
    //     let a = async { Ok::<i32, i32>(1) };
    //     let b = async { Ok::<i32, i32>(2) };
    //     // assert_eq!(try_join!(a, b), Ok((1, 2)));
    //     println!("{:?}", try_join!(a, b).unwrap());
    //     // `try_join!` is variadic, so you can pass any number of futures
    //     let c = async { Ok::<i32, i32>(3) };
    //     let d = async { Ok::<i32, i32>(4) };
    //     let e = async { Ok::<i32, i32>(5) };
    //     assert_eq!(try_join!(c, d, e), Ok((3, 4, 5)));
    // });
    Ok(())
}
