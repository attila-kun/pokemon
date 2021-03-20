mod species_url_request;
mod species_description_request;
mod json_request;

pub async fn get_shakespearean_description(pokemon_name: &str) -> Result<String, ()> {
    let pokemon_ability_urls_result = species_url_request::get_pokemon_species_url(pokemon_name).await;
    match pokemon_ability_urls_result {
        Ok(url) => {
            let species_description_result = species_description_request::get_species_description(&url).await;
            species_description_result
        },
        Err(()) => Err(())
    }
}