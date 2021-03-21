mod shakespearean_translation_request;
mod species_url_request;
mod species_description_request;
mod json_request;
use crate::json_request::{NetworkJsonRequest};
use json_request::JsonRequest;
use shakespearean_translation_request::get_shakespearean_translation;
use species_description_request::get_species_description;
use species_url_request::get_pokemon_species_url;

async fn get_shakespearean_description_<R: JsonRequest>(pokemon_name: &str) -> Result<String, ErrorReason> {
    let pokemon_species_url_result = get_pokemon_species_url::<R>(pokemon_name).await;
    let description_result = match pokemon_species_url_result {
        Ok(url) => get_species_description::<R>(&url).await,
        Err(json_request::JsonErrorReason::BadBody(body)) => {
            match body.as_ref() {
                "Not Found" => return Err(ErrorReason::NotFound),
                _ => return Err(ErrorReason::Unknown)
            }
        },
        Err(error) => Err(error)
    };
    let shakespearean_translation_result = match description_result {
        Ok(description) => get_shakespearean_translation::<R>(&description).await,
        Err(error) => Err(error)
    };
    match shakespearean_translation_result {
        Ok(translation) => Ok(translation),
        Err(_) => Err(ErrorReason::Unknown)
    }
}

pub async fn get_shakespearean_description(pokemon_name: &str) -> Result<String, ErrorReason> {
    get_shakespearean_description_::<NetworkJsonRequest>(pokemon_name).await
}

pub enum ErrorReason {
    NotFound,
    Unknown
}

mod tests {
    use super::*;
    use async_trait::async_trait;
    use crate::json_request::{JsonErrorReason};
    struct MockJsonRequest;

    #[async_trait(?Send)]
    impl JsonRequest for MockJsonRequest {

        async fn get_json_response<T: serde::de::DeserializeOwned>(request_url: &str) -> Result<T, JsonErrorReason> {

            let json_text = {
                if request_url.starts_with("https://pokeapi.co/api/v2/pokemon/") {
                    "{
                        \"species\": {
                        \"name\": \"charizard\",
                        \"url\": \"https://pokeapi.co/api/v2/pokemon-species/6/\"
                        }
                    }"
                } else if request_url.starts_with("https://pokeapi.co/api/v2/pokemon-species/") {
                    "
                    {
                    \"flavor_text_entries\": [
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
                    "
                } else if request_url.starts_with("https://api.funtranslations.com/translate/shakespeare.json") {
                    "{
                        \"success\": {
                            \"total\": 1
                        },
                        \"contents\": {
                            \"translated\": \"Thee did giveth mr. Tim a hearty meal, but unfortunately what he did doth englut did maketh him kicketh the bucket.\",
                            \"text\": \"You gave Mr. Tim a hearty meal, but unfortunately what he ate made him die.\",
                            \"translation\": \"shakespeare\"
                        }
                    }
                    "
                }
                else {
                    unimplemented!()
                }
            };

            Ok(serde_json::from_str(&json_text).unwrap())
        }
    }

    struct MockNotFoundJsonRequest;

    #[async_trait(?Send)]
    impl JsonRequest for MockNotFoundJsonRequest {

        async fn get_json_response<T: serde::de::DeserializeOwned>(request_url: &str) -> Result<T, JsonErrorReason> {

            if request_url.starts_with("https://pokeapi.co/api/v2/pokemon/") {
                Err(JsonErrorReason::BadBody(String::from("Not Found")))
            }
            else {
                unimplemented!()
            }
        }
    }

    #[actix_rt::test]
    async fn test_returns_shakespearan_description() {
        let translation_result = get_shakespearean_description_::<MockJsonRequest>("pokemon").await;
        match translation_result {
            Ok(translation) => assert_eq!(translation, "Thee did giveth mr. Tim a hearty meal, but unfortunately what he did doth englut did maketh him kicketh the bucket."),
            Err(_) => assert!(false)
        }
    }

    #[actix_rt::test]
    async fn test_returns_not_found_error() {
        let translation_result = get_shakespearean_description_::<MockNotFoundJsonRequest>("pokemon").await;
        match translation_result {
            Ok(_) => assert!(false),
            Err(ErrorReason::NotFound) => assert!(true),
            Err(_) => assert!(false)
        }
    }

  }