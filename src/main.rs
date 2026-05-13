use actix_web::{get, App, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    "Backend running!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string());

    println!("Starting server on port {}", port);

    HttpServer::new(|| {
        App::new()
            .service(hello)
    })
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await
}