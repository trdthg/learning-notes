use tokio::net::TcpListener;

mod handler;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("mini-server started on 127.0.0.1:8080...");

    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    loop {
        let (mut stream, _) = listener.accept().await?;
        println!("new addr has connected!");
        tokio::spawn(async move {
            handler::handle_request(stream).await;
        });
    }
    Ok(())
}
