use actix_web::{App, HttpResponse, HttpServer, Result, get, web};
use pokemon_core;

#[get("/pokemon/{name}")]
async fn pokemon(info: web::Path<(String)>) -> HttpResponse {
    println!("{}", &info.0);
    HttpResponse::Ok()
        .content_type("application/json")
        .body("something")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| { App::new().service(pokemon) })
    .bind("0.0.0.0:5000")?
    .run()
    .await
}