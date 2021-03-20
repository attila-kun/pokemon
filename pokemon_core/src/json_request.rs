use actix_web::client::Client;
use serde_json;

pub async fn get_json_response<T: serde::de::DeserializeOwned>(url: &str) -> Result<T, ()> {

  let client = Client::default();
  let initial_response = client.get(url)
      .header("User-Agent", "actix-web/3.3.2")
      .send()
      .await;

  let body_response = match initial_response {
      Ok(mut response) => response.body().await,
      Err(_) => {
          eprint!("Error requesting URL: {}", url);
          return Err(());
      }
  };

  let body_utf8_parse_result = match body_response {
      Ok(body_response_bytes) => String::from_utf8(body_response_bytes.to_vec()),
      Err(_) => {
          eprint!("Error while receiving body for URL: {}", url);
          return Err(());
      }
  };

  let pokemon_json_parse_result: serde_json::Result<T> = match body_utf8_parse_result {
      Ok(body_utf8_string) => serde_json::from_str(&body_utf8_string),
      Err(_) => {
          eprint!("Error while parsing body into UTF8 string for URL: {}", url);
          return Err(());
      }
  };

  match pokemon_json_parse_result {
      Ok(pokemon_json) => Ok(pokemon_json),
      Err(error) => {
          eprint!("Error while parsing UTF8 string into JSON for URL: {}. Error: {}", url, error.to_string());
          return Err(());
      }
  }
}