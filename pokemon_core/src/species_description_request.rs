use crate::json_request::get_json_response;
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

pub async fn get_species_description(specis_url: &str) -> Result<String, ()> {

  match get_json_response::<SpeciesJson>(specis_url).await {
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