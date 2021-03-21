use actix_web::{Result, get, web};
use actix_web::error::{ErrorInternalServerError, ErrorNotFound};
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
    let result = get_shakespearean_description(pokemon_name).await;
    match result {
        Ok(description) => Ok(web::Json(
            PokemonResponse {
                name: String::from(pokemon_name),
                description: description
            }
        )),
        Err(pokemon_core::ErrorReason::NotFound) => Err(ErrorNotFound("Pokemon not found.")),
        Err(_) => Err(ErrorInternalServerError("Error while generating Shakespearean description."))
    }

}

// This is to aid testing in a real-life scenario where more than one services are set up.
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(pokemon);
}
