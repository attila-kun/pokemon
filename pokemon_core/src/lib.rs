mod ability_urls;

pub async fn get_shakespearean_description(pokemon_name: &str) -> Result<String, ()> {
    let pokemon_ability_urls_result = ability_urls::get_pokemon_ability_urls(pokemon_name).await;
    match pokemon_ability_urls_result {
        Ok(urls) => todo!(),
        Err(()) => Err(())
    }
}