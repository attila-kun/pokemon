mod shakespearean_translation_request;
mod species_url_request;
mod species_description_request;
mod json_request;

pub async fn get_shakespearean_description(pokemon_name: &str) -> Result<String, ()> {
    let pokemon_species_url_result = species_url_request::get_pokemon_species_url(pokemon_name).await;
    let description_result = match pokemon_species_url_result {
        Ok(url) => species_description_request::get_species_description(&url).await,
        Err(()) => Err(())
    };
    match description_result {
        Ok(desrciption) => shakespearean_translation_request::get_shakespearean_translation(&desrciption).await,
        Err(_) => Err(())
    }
}