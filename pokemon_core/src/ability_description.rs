use actix_web::client::Client;
use serde::{Deserialize};
use serde_json;
use url;

pub async fn get_ability_description(ability_url: &str) {
  let mut client = Client::default();
  let initial_response = client.get(ability_url)
     .header("User-Agent", "actix-web/3.3.2")
     .send()
     .await;
}