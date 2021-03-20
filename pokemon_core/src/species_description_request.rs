use crate::json_request::{JsonRequest};
use serde::{Deserialize};

#[derive(Deserialize)]
struct Language {
  name: String,
}

#[derive(Deserialize)]
struct Version {
  name: String,
}

#[derive(Deserialize)]
struct FlavorTextEntry {
  flavor_text: String,
  language: Language,
  version: Version
}

#[derive(Deserialize)]
struct SpeciesJson {
  flavor_text_entries: Vec<FlavorTextEntry>
}

pub async fn get_species_description<R: JsonRequest>(species_url: &str) -> Result<String, ()> {

  match R::get_json_response::<SpeciesJson>(species_url).await {
    Ok(species_json) => {
      let first_english_flavor_text_entry = species_json.flavor_text_entries
        .iter()
        .find(|flavor_text_entry| { flavor_text_entry.language.name == "en" && flavor_text_entry.version.name == "emerald" });

      match first_english_flavor_text_entry {
        Some(flavor_text_entry) => Ok(flavor_text_entry.flavor_text.clone()),
        None => Err(())
      }
    },
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
            \"flavor_text_entries\": [
              {
                \"flavor_text\": \"CHARIZARD flies around the sky in\\nsearch of powerful opponents.\\nIt breathes fire of such great heat\\fthat it melts anything. However, it\\nnever turns its fiery breath on any\\nopponent weaker than itself.\",
                \"language\": {
                  \"name\": \"en\",
                  \"url\": \"https://pokeapi.co/api/v2/language/9/\"
                },
                \"version\": {
                  \"name\": \"sapphire\",
                  \"url\": \"https://pokeapi.co/api/v2/version/8/\"
                }
              },
              {
                \"flavor_text\": \"Charizard se dedica a volar por los cielos en busca de\\noponentes fuertes. Echa fuego por la boca y es capaz de\\nderretir cualquier cosa. No obstante, si su rival es más débil\\nque él, no usará este ataque.\",
                \"language\": {
                  \"name\": \"es\",
                  \"url\": \"https://pokeapi.co/api/v2/language/7/\"
                },
                \"version\": {
                  \"name\": \"emerald\",
                  \"url\": \"https://pokeapi.co/api/v2/version/26/\"
                }
              },
              {
                \"flavor_text\": \"A CHARIZARD flies about in search of\\nstrong opponents. It breathes intense\\nflames that can melt any material. However,\\nit will never torch a weaker foe.\",
                \"language\": {
                  \"name\": \"en\",
                  \"url\": \"https://pokeapi.co/api/v2/language/9/\"
                },
                \"version\": {
                  \"name\": \"emerald\",
                  \"url\": \"https://pokeapi.co/api/v2/version/9/\"
                }
              }
            ]
          }
          ";

          Ok(serde_json::from_str(&json_text).unwrap())
      }
  }

  #[actix_rt::test]
  async fn test_returns_first_english_emerald_description_from_json_response() {
    let description_result = get_species_description::<MockJsonRequest>("Mock Pokemon species URL").await;
    match description_result {
      Ok(description) => assert_eq!(description, "A CHARIZARD flies about in search of\nstrong opponents. It breathes intense\nflames that can melt any material. However,\nit will never torch a weaker foe."),
      Err(_) => assert!(false)
    }
  }

  #[actix_rt::test]
  async fn test_returns_err_if_json_request_fails() {
    let description_result = get_species_description::<json_request::test_helper::MockFailedJsonRequest>("https://pokeapi.co/api/v2/pokemon/charizard").await;
    match description_result {
      Ok(_) => assert!(false),
      Err(_) => assert!(true)
    }
  }

}