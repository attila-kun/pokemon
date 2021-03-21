mod shakespearean_translation_request;
mod species_url_request;
mod species_description_request;
mod json_request;
use crate::json_request::{NetworkJsonRequest};
use shakespearean_translation_request::get_shakespearean_translation;
use species_description_request::get_species_description;
use species_url_request::get_pokemon_species_url;

pub async fn get_shakespearean_description(pokemon_name: &str) -> Result<String, ErrorReason> {
    let pokemon_species_url_result = get_pokemon_species_url::<NetworkJsonRequest>(pokemon_name).await;
    let description_result = match pokemon_species_url_result {
        Ok(url) => get_species_description::<NetworkJsonRequest>(&url).await,
        Err(json_request::JsonErrorReason::BadBody(body)) => {
            match body.as_ref() {
                "Not Found" => return Err(ErrorReason::NotFound),
                _ => return Err(ErrorReason::Unknown)
            }
        },
        Err(error) => Err(error)
    };
    let shakespearean_translation_result = match description_result {
        Ok(description) => get_shakespearean_translation::<NetworkJsonRequest>(&description).await,
        Err(error) => Err(error)
    };
    match shakespearean_translation_result {
        Ok(translation) => Ok(translation),
        Err(_) => Err(ErrorReason::Unknown)
    }
}

pub enum ErrorReason {
    NotFound,
    Unknown
}