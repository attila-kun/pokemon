mod ability_urls;
mod ability_description;
mod json_request;

pub async fn get_shakespearean_description(pokemon_name: &str) -> Result<String, ()> {
    let pokemon_ability_urls_result = ability_urls::get_pokemon_ability_urls(pokemon_name).await;
    match pokemon_ability_urls_result {
        Ok(urls) => {
            let url = urls.get(0).unwrap();
            println!("{:?}", ability_description::get_ability_description(url).await);
            todo!()
        },
        Err(()) => Err(())
    }
}