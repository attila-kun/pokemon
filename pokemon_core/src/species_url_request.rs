use serde::{Deserialize};
use url;
use crate::json_request::{JsonRequest};

#[derive(Deserialize)]
struct Species {
    url: String
}

#[derive(Deserialize)]
struct PokemonJson {
    species: Species
}

pub async fn get_pokemon_species_url<R: JsonRequest>(pokemon_name: &str) -> Result<String, ()> {

    let api_base_url = url::Url::parse("https://pokeapi.co/api/v2/pokemon/").unwrap();

    let api_url = match api_base_url.join(pokemon_name) {
        Ok(url) => url.to_string(),
        Err(_) => {
            eprint!("Error creating API URL for Pokemon: {}", pokemon_name);
            return Err(());
        }
    };

    match R::get_json_response::<PokemonJson>(&api_url).await {
        Ok(pokemon_json) => Ok(pokemon_json.species.url),
        Err(_) => Err(())
    }
}

mod tests {
    use super::*;
    use async_trait::async_trait;
    use crate::json_request;

    struct MockJsonRequest;

    #[async_trait(?Send)]
    impl JsonRequest for MockJsonRequest {
        async fn get_json_response<T: serde::de::DeserializeOwned>(_request_url: &str) -> Result<T, ()> {
            let json_text = "
            {
                \"species\": {
                  \"name\": \"charizard\",
                  \"url\": \"https://pokeapi.co/api/v2/pokemon-species/6/\"
                }
            }
            ";

            Ok(serde_json::from_str(&json_text).unwrap())
        }
    }

    #[actix_rt::test]
    async fn test_returns_species_url_from_json_response() {
      let species_url_result = get_pokemon_species_url::<MockJsonRequest>("Mock Pokemon URL").await;
      match species_url_result {
        Ok(url) => assert_eq!(url, "https://pokeapi.co/api/v2/pokemon-species/6/"),
        Err(_) => assert!(false)
      }
    }

    #[actix_rt::test]
    async fn test_returns_err_if_json_request_fails() {
      let description_result = get_pokemon_species_url::<json_request::test_helper::MockFailedJsonRequest>("Mock Pokemon URL").await;
      match description_result {
        Ok(_) => assert!(false),
        Err(_) => assert!(true)
      }
    }

  }