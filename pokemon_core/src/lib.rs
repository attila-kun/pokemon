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

async fn get_pokemon_ability_urls(pokemon_name: &str) -> Result<Vec<String>, String> {

    let api_base_url = url::Url::parse("https://pokeapi.co/api/v2/pokemon/").unwrap();
    let api_url_result = api_base_url.join(pokemon_name);

    let api_url = match api_url_result {
        Ok(url) => url.to_string(),
        Err(_) => return Err(String::from("Error creating API URL."))
    };

    let mut client = Client::default();
    let initial_response = client.get(api_url)
       .header("User-Agent", "actix-web/3.3.2")
       .send()
       .await;

    let body_response = match initial_response {
        Ok(mut response) => response.body().await,
        Err(_) => return Err(String::from("Error requesting Pokemon data."))
    };

    let body_utf8_parse_result = match body_response {
        Ok(body_response_bytes) => String::from_utf8(body_response_bytes.to_vec()),
        Err(_) => return Err(String::from("Error while receiving Pokemon data body."))
    };

    let pokemon_json_parse_result: serde_json::Result<PokemonJson> = match body_utf8_parse_result {
        Ok(body_utf8_string) => serde_json::from_str(&body_utf8_string),
        Err(_) => return Err(String::from("Error while parsing body into UTF8 string."))
    };

    match pokemon_json_parse_result {
        Ok(pokemon_json) => {
            Ok(pokemon_json.abilities
                .iter()
                .map(|ability_item| { ability_item.ability.url.clone() })
                .collect())
        },
        Err(error) => Err(error.to_string())
    }
}

pub async fn get_shakespearean_description(pokemon_name: &str) -> String {
    let result = get_pokemon_ability_urls(pokemon_name).await;
    println!("result {:?}", result);
    String::new()
}