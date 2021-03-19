use actix_web::{App, HttpServer, Result, get, web};
use pokemon_core::get_shakespearean_description;
use serde::{Serialize};

#[derive(Serialize)]
struct PokemonResponse {
    name: String,
    description: String
}

#[get("/pokemon/{name}")]
async fn pokemon(info: web::Path<(String)>) -> Result<web::Json<PokemonResponse>> {
    let pokemon_name = &info.0;
    Ok(web::Json(PokemonResponse {
        name: String::from(pokemon_name),
        description: get_shakespearean_description(String::from(pokemon_name))
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| { App::new().service(pokemon) })
    .bind("0.0.0.0:5000")?
    .run()
    .await
}