use crate::json_request::get_json_response;
use serde::{Deserialize};

#[derive(Deserialize)]
struct Language {
  name: String,
}

#[derive(Deserialize)]
struct FlavorTextEntry {
  flavor_text: String,
  language: Language
}

#[derive(Deserialize)]
struct AbilitiesJson {
  flavor_text_entries: Vec<FlavorTextEntry>
}

pub async fn get_ability_description(ability_url: &str) -> Result<String, ()> {

  match get_json_response::<AbilitiesJson>(ability_url).await {
    Ok(abilities_json) => {
      let first_english_flavor_text_entry = abilities_json.flavor_text_entries
        .iter()
        .find(|flavor_text_entry| { flavor_text_entry.language.name == "en" });

      match first_english_flavor_text_entry {
        Some(flavor_text_entry) => Ok(flavor_text_entry.flavor_text.clone()),
        None => Err(())
      }
    },
    Err(_) => Err(())
  }
}