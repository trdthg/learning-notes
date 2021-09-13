//

#[tokio::main]
async fn main() {
    use tokio::fs::File;
    use tokio::io::AsyncReadExt;

    // let mut f = File::open("Cargo.toml").await.unwrap();
    // let mut buf = [0; 1024];
    // let n = f.read(&mut buf).await.unwrap();
    // println!("{}", String::from_utf8_lossy(&buf));

    let mut reader: &[u8] = b"hello";
    let mut file = File::create("foo.txt").await.unwrap();

    tokio::io::copy(&mut reader, &mut file).await.unwrap();
}
