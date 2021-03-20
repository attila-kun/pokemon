use serde::{Deserialize};
use url;
use crate::json_request::get_json_response;

#[derive(Deserialize)]
struct PokemonAbility {
    url: String
}

#[derive(Deserialize)]
struct PokemonAbilityItem {
    ability: PokemonAbility
}

#[derive(Deserialize)]
struct PokemonJson {
    abilities: Vec<PokemonAbilityItem>
}

pub async fn get_pokemon_ability_urls(pokemon_name: &str) -> Result<Vec<String>, ()> {

    let api_base_url = url::Url::parse("https://pokeapi.co/api/v2/pokemon/").unwrap();

    let api_url = match api_base_url.join(pokemon_name) {
        Ok(url) => url.to_string(),
        Err(_) => {
            eprint!("Error creating API URL for Pokemon: {}", pokemon_name);
            return Err(());
        }
    };

    match get_json_response::<PokemonJson>(&api_url).await {
        Ok(pokemon_json) => {
            Ok(pokemon_json.abilities
                .iter()
                .map(|ability_item| { ability_item.ability.url.clone() })
                .collect())
        },
        Err(_) => Err(())
    }
}