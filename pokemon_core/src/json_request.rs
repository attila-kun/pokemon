use async_trait::async_trait;
use actix_web::client::Client;
use serde_json;

#[async_trait(?Send)]
pub trait JsonRequest {
    async fn get_json_response<T: serde::de::DeserializeOwned>(url: &str) -> Result<T, JsonErrorReason>;
}

pub struct NetworkJsonRequest;

#[async_trait(?Send)]
impl JsonRequest for NetworkJsonRequest {
    // Performs a GET request against the provided URL and parses the result into the provided type argument T.
    async fn get_json_response<T: serde::de::DeserializeOwned>(url: &str) -> Result<T, JsonErrorReason> {

        let client = Client::default();
        let initial_response = client.get(url)
            .header("User-Agent", "actix-web/3.3.2")
            .send()
            .await;

        let body_response = match initial_response {
            Ok(mut response) => response.body().await,
            Err(_) => {
                eprintln!("Error requesting URL: {}", url);
                return Err(JsonErrorReason::Unknown);
            }
        };

        let body_utf8_parse_result = match body_response {
            Ok(body_response_bytes) => String::from_utf8(body_response_bytes.to_vec()),
            Err(_) => {
                eprintln!("Error while receiving body for URL: {}", url);
                return Err(JsonErrorReason::Unknown);
            }
        };

        let (body_utf8_string, pokemon_json_parse_result): (String, serde_json::Result<T>) = match body_utf8_parse_result {
            Ok(body_utf8_string) => (body_utf8_string.clone(), serde_json::from_str(&body_utf8_string)),
            Err(_) => {
                eprintln!("Error while parsing body into UTF8 string for URL: {}", url);
                return Err(JsonErrorReason::Unknown);
            }
        };

        match pokemon_json_parse_result {
            Ok(pokemon_json) => Ok(pokemon_json),
            Err(error) => {
                eprintln!("Error while parsing UTF8 string into JSON for URL: {}.\n\tError: {}.\n\tBody string: {}", url, error.to_string(), body_utf8_string);
                return Err(JsonErrorReason::BadBody(body_utf8_string));
            }
        }
    }
}

pub enum JsonErrorReason {
    BadBody(String),
    Unknown
}

pub mod test_helper {
    use super::*;
    pub struct MockFailedJsonRequest;

    #[async_trait(?Send)]
    impl JsonRequest for MockFailedJsonRequest {
        async fn get_json_response<T: serde::de::DeserializeOwned>(_request_url: &str) -> Result<T, JsonErrorReason> {
            Err(JsonErrorReason::Unknown) // useful for testing the behaviour of services whose network request fails
        }
    }
}