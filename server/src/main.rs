use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let listen_address = "0.0.0.0:5000";
    let server = HttpServer::new(|| { App::new().configure(server::config) })
        .bind(listen_address)?
        .run();

    println!("Pokemon service configuration completed. Listening on address {}", listen_address);
    server.await
}