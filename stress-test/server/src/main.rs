use actix_web::{get, web, App, HttpServer, Responder};

#[get("/{id}/{name}/index.html")]
async fn index(web::Path((id, name)): web::Path<(u32, String)>) -> impl Responder {
    format!("Hello {}! id:{}", name, id)
}

#[get("/")]
async fn hello() -> impl Responder {
    format!("Hello")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    print!("Hello");
    HttpServer::new(|| App::new().service(index))
        .bind("127.0.0.1:8081")?
        .run()
        .await
}