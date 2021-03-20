use crate::json_request::get_json_response;
use serde::{Deserialize};

#[derive(Deserialize)]
struct Contents {
  text: String
}

#[derive(Deserialize)]
struct TranslationJson {
  contents: Contents
}

pub async fn get_shakespearean_translation(text: &str) -> Result<String, ()> {

  let mut api_base_url = url::Url::parse("https://api.funtranslations.com/translate/shakespeare.json").unwrap();
  {
    let mut api_base_url_query_pairs = api_base_url.query_pairs_mut();
    api_base_url_query_pairs.append_pair("text", text);
  }

  match get_json_response::<TranslationJson>(&api_base_url.to_string()).await {
    Ok(translation_json) => Ok(translation_json.contents.text),
    Err(_) => Err(())
  }
}