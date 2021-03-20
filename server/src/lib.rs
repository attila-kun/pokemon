use actix_web::{Result, get, web};
use pokemon_core::get_shakespearean_description;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct PokemonResponse {
    pub name: String,
    pub description: String
}

#[get("/pokemon/{name}")]
async fn pokemon(info: web::Path<String>) -> Result<web::Json<PokemonResponse>> {
    let pokemon_name = &info.0;
    Ok(web::Json(PokemonResponse {
        name: String::from(pokemon_name),
        description: get_shakespearean_description(String::from(pokemon_name)).await
    }))
}

// This is to aid testing in a real-life scenario where more than one services are set up.
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(pokemon);
}
