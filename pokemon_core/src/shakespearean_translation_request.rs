use crate::{json_request::{JsonErrorReason, JsonRequest}};
use regex::Regex;
use serde::{Deserialize};

#[derive(Deserialize)]
struct Contents {
  translated: String
}

#[derive(Deserialize)]
struct TranslationJson {
  contents: Contents
}

fn replace_whitespace_with_space(text: &str) -> String {
  let re = Regex::new(r"\s+").unwrap();
  re.replace_all(text, " ").to_string()
}

pub async fn get_shakespearean_translation<R: JsonRequest>(text: &str) -> Result<String, JsonErrorReason> {

  let mut api_base_url = url::Url::parse("https://api.funtranslations.com/translate/shakespeare.json").unwrap();
  {
    let mut api_base_url_query_pairs = api_base_url.query_pairs_mut();
    api_base_url_query_pairs.append_pair("text", &replace_whitespace_with_space(&text));
  }

  match R::get_json_response::<TranslationJson>(&api_base_url.to_string()).await {
    Ok(translation_json) => Ok(translation_json.contents.translated),
    Err(error) => Err(error)
  }
}

mod tests {
  use super::*;
  use async_trait::async_trait;
  use crate::json_request;
  use std::borrow::Cow;
  use url;

struct MockJsonRequest;

#[async_trait(?Send)]
impl JsonRequest for MockJsonRequest {
    async fn get_json_response<T: serde::de::DeserializeOwned>(request_url: &str) -> Result<T, json_request::JsonErrorReason> {
        let json_text = "{
            \"success\": {
                \"total\": 1
            },
            \"contents\": {
                \"translated\": \"Thee did giveth mr. Tim a hearty meal, but unfortunately what he did doth englut did maketh him kicketh the bucket.\",
                \"text\": \"You gave Mr. Tim a hearty meal, but unfortunately what he ate made him die.\",
                \"translation\": \"shakespeare\"
            }
        }
        ";

        let url = url::Url::parse(request_url).unwrap();
        let mut query_pairs = url.query_pairs();
        assert_eq!(query_pairs.count(), 1);
        // checks the removal of whitespace
        assert_eq!(query_pairs.next(), Some((Cow::Borrowed("text"), Cow::Borrowed("First line second line"))));

        Ok(serde_json::from_str(&json_text).unwrap())
    }
}

  #[actix_rt::test]
  async fn test_returns_shakespearan_translation_from_json_response() {
    let translation_result = get_shakespearean_translation::<MockJsonRequest>("First line \n second line").await;
    match translation_result {
      Ok(translation) => assert_eq!(translation, "Thee did giveth mr. Tim a hearty meal, but unfortunately what he did doth englut did maketh him kicketh the bucket."),
      Err(_) => assert!(false)
    }
  }

  #[actix_rt::test]
  async fn test_returns_err_if_json_request_fails() {
    let description_result = get_shakespearean_translation::<json_request::test_helper::MockFailedJsonRequest>("First line \n second line").await;
    match description_result {
      Ok(_) => assert!(false),
      Err(_) => assert!(true)
    }
  }

}