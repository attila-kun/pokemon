use actix_web::client::Client;

async fn make_request(url: &str) -> String {

    let mut client = Client::default();
    let response = client.get(url.to_string())
       .header("User-Agent", "actix-web/3.3.2")
       .send()
       .await;

    let body = String::from_utf8(response.unwrap().body().await.unwrap().to_vec()).unwrap();
    body
}

pub async fn get_shakespearean_description(pokemon_name: String) -> String {
    make_request("https://pokeapi.co/api/v2/pokemon/ditto").await
}