mod shakespearean_translation_request;
mod species_url_request;
mod species_description_request;
mod json_request;
use crate::json_request::{NetworkJsonRequest};
use shakespearean_translation_request::get_shakespearean_translation;
use species_description_request::get_species_description;
use species_url_request::get_pokemon_species_url;

pub async fn get_shakespearean_description(pokemon_name: &str) -> Result<String, ()> {
    let pokemon_species_url_result = get_pokemon_species_url::<NetworkJsonRequest>(pokemon_name).await;
    let description_result = match pokemon_species_url_result {
        Ok(url) => get_species_description::<NetworkJsonRequest>(&url).await,
        Err(()) => Err(())
    };
    match description_result {
        Ok(desrciption) => get_shakespearean_translation::<NetworkJsonRequest>(&desrciption).await,
        Err(_) => Err(())
    }
}