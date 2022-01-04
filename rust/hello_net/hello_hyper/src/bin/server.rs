use std::convert::Infallible;
use std::net::SocketAddr;

use hyper::{Body, Request, Response, Server};
use hyper::service;

#[tokio::main]
async fn main() {
    // hello().await;

    let addr = ([127, 0, 0, 1], 3000).into();
    let make_svc = service::make_service_fn(|_conn| async {
        Ok::<_, hyper::Error>(service::service_fn(routing))
    });

    let server = Server::bind(&addr).serve(make_svc);

    // if let Err(e) = server.await {
    //     eprintln!("server error: {}", e);
    // }

    // And now add a graceful shutdown signal...
    let graceful = server.with_graceful_shutdown(shutdown_singal());
    if let Err(e) = graceful.await {
        eprintln!("server error: {}", e);
    }
}

async fn hello() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let make_svc = service::make_service_fn(|_conn| async {
        Ok::<_, Infallible>(service::service_fn(hello_world))
        // Ok::<_, Infallible>(service::service_fn(routing))
    });

    let server = Server::bind(&addr).serve(make_svc);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

async fn hello_world(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new("Hello, World".into()))
}

async fn routing(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    use hyper::{ Method, StatusCode};
    use futures::TryStreamExt as _;

    let mut response = Response::new(Body::empty());
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => {
            *response.body_mut() = Body::from("Try POSTing data to /echo");
        }
        (&Method::POST, "/echo") => {
            *response.body_mut() = req.into_body();
        }
        (&Method::POST, "/echo/uppercase") => {
            let mapping = req
                .into_body()
                .map_ok(|chunk| {
                    chunk.iter()
                        .map(|byte| byte.to_ascii_uppercase())
                        .collect::<Vec<u8>>()
                });
            *response.body_mut() = Body::wrap_stream(mapping);
        }
        (&Method::POST, "/echo/reverse") => {
            // Await the full body to be concatenated into a single `Bytes`...
            let full_body = hyper::body::to_bytes(req.into_body()).await?;

            // Iterate the full body in reverse order and collect into a new Vec.
            let reversed = full_body.iter()
                .rev()
                .cloned()
                .collect::<Vec<u8>>();

            *response.body_mut() = reversed.into();
        },
        _ => {
            *response.status_mut() = StatusCode::NOT_FOUND;
        }
    }
    Ok(response)
}

async fn shutdown_singal() {
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install CTRL+C signal handler")
}