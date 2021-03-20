use crate::json_request::get_json_response;
use serde::{Deserialize};

#[derive(Deserialize)]
struct FlavorTextEntry {
  flavor_text: String,
}

#[derive(Deserialize)]
struct AbilitiesJson {
  flavor_text_entries: Vec<FlavorTextEntry>
}

pub async fn get_ability_description(ability_url: &str) -> Result<Vec<String>, ()> {

  match get_json_response::<AbilitiesJson>(ability_url).await {
    Ok(abilities_json) => Ok(abilities_json.flavor_text_entries
      .iter()
      .map(|flavor_text_entry| { flavor_text_entry.flavor_text.clone() })
      .collect()
    ),
    Err(_) => Err(())
  }
}