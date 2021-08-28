
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    hello().await?;
    making_post().await?;
    multipart_requests().await?;
    Ok(())
}

async fn hello() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    use hyper::Client;
    use hyper::body::HttpBody as _;
    use tokio::io::{stdout, AsyncWriteExt as _};

    // Still inside `async fn main`...
    let client = Client::new();

    // Parse an `http::Uri`...
    let uri = "http://httpbin.org/ip".parse()?;

    // Await the response...
    let mut resp = client.get(uri).await?;

    println!("Response: {}", resp.status());

    // And now...
    while let Some(chunk) = resp.body_mut().data().await {
        stdout().write_all(&chunk?).await?;
    }
    Ok(())
}

async fn making_post() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    use hyper::{Body, Method, Request, Uri};
    use hyper::Client;

    let req = Request::builder()
        .method(Method::POST)
        .uri("http://httpbin.org/post")
        .header("content-type", "application/json")
        .body(Body::from(r#"{"library":"hyper"}"#))?;

    let client = Client::new();
    let resp = client.request(req).await?;
    println!("Response: {}", resp.status());
    Ok(())
}

async fn multipart_requests() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    use hyper::Client;
    use hyper::{Body, Method, Request, Uri};

    let client = Client::new();

    let ip_fut = async {
        let resp = client.get(Uri::from_static("http://httpbin.org/ip")).await?;
        hyper::body::to_bytes(resp.into_body()).await
    };

    let headers_fut = async {
        let resp = client.get(Uri::from_static("http://httpbin.org/headers")).await?;
        hyper::body::to_bytes(resp.into_body()).await
    };

    let (ip, headers) = futures::try_join!(ip_fut, headers_fut)?;
    println!("{:?}\n{:#?}", ip, headers);
    Ok(())
}