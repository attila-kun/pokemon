use serde::{Deserialize};
use url;
use crate::json_request::get_json_response;

#[derive(Deserialize)]
struct Species {
    url: String
}

#[derive(Deserialize)]
struct PokemonJson {
    species: Species
}

pub async fn get_pokemon_species_url(pokemon_name: &str) -> Result<String, ()> {

    let api_base_url = url::Url::parse("https://pokeapi.co/api/v2/pokemon/").unwrap();

    let api_url = match api_base_url.join(pokemon_name) {
        Ok(url) => url.to_string(),
        Err(_) => {
            eprint!("Error creating API URL for Pokemon: {}", pokemon_name);
            return Err(());
        }
    };

    match get_json_response::<PokemonJson>(&api_url).await {
        Ok(pokemon_json) => Ok(pokemon_json.species.url),
        Err(_) => Err(())
    }
}