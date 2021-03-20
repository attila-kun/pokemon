use actix_web::client::Client;
use serde::{Deserialize};
use serde_json;
use url;

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

async fn get_pokemon_ability_urls(pokemon_name: &str) -> Result<Vec<String>, ()> {

    let api_base_url = url::Url::parse("https://pokeapi.co/api/v2/pokemon/").unwrap();
    let api_url_result = api_base_url.join(pokemon_name);

    let api_url = match api_url_result {
        Ok(url) => url.to_string(),
        Err(_) => {
            eprint!("Error creating API URL for Pokemon: {}", pokemon_name);
            return Err(());
        }
    };

    let mut client = Client::default();
    let initial_response = client.get(api_url)
       .header("User-Agent", "actix-web/3.3.2")
       .send()
       .await;

    let body_response = match initial_response {
        Ok(mut response) => response.body().await,
        Err(_) => {
            eprint!("Error requesting Pokemon data for Pokemon: {}", pokemon_name);
            return Err(());
        }
    };

    let body_utf8_parse_result = match body_response {
        Ok(body_response_bytes) => String::from_utf8(body_response_bytes.to_vec()),
        Err(_) => {
            eprint!("Error while receiving Pokemon data body for Pokemon: {}", pokemon_name);
            return Err(());
        }
    };

    let pokemon_json_parse_result: serde_json::Result<PokemonJson> = match body_utf8_parse_result {
        Ok(body_utf8_string) => serde_json::from_str(&body_utf8_string),
        Err(_) => {
            eprint!("Error while parsing body into UTF8 string for Pokemon: {}", pokemon_name);
            return Err(());
        }
    };

    match pokemon_json_parse_result {
        Ok(pokemon_json) => {
            Ok(pokemon_json.abilities
                .iter()
                .map(|ability_item| { ability_item.ability.url.clone() })
                .collect())
        },
        Err(error) => {
            eprint!("Error while parsing UTF8 string into JSON for Pokemon {}. Error: {}", pokemon_name, error.to_string());
            return Err(());
        }
    }
}

pub async fn get_shakespearean_description(pokemon_name: &str) -> Result<String, ()> {
    let pokemon_ability_urls_result = get_pokemon_ability_urls(pokemon_name).await;
    match pokemon_ability_urls_result {
        Ok(urls) => todo!(),
        Err(()) => Err(())
    }
}