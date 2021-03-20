mod ability_urls;
mod ability_description;
mod json_request;
use futures::future::join_all;

pub async fn get_shakespearean_description(pokemon_name: &str) -> Result<String, ()> {
    let pokemon_ability_urls_result = ability_urls::get_pokemon_ability_urls(pokemon_name).await;
    match pokemon_ability_urls_result {
        Ok(urls) => {
            let futures = urls.iter().map(|url| { ability_description::get_ability_description(url) });
            let ability_description_results = join_all(futures).await;
            let ability_descriptions: Vec<String> = ability_description_results
                .into_iter()
                .filter(|result| { result.is_ok() })
                .map(|result| { result.unwrap() })
                .collect();

            println!("{:?}", ability_descriptions);
            todo!()
        },
        Err(()) => Err(())
    }
}