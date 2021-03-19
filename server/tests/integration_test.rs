use actix_web::{test, web, App};
use server::{config, PokemonResponse};

#[actix_rt::test]
async fn test_index_get() {
    let mut app = test::init_service(App::new().configure(config)).await;
    let request = test::TestRequest::with_uri("/pokemon/pikachu").to_request();
    let response = test::call_service(&mut app, request).await;

    assert!(response.status().is_success());

    let pokemon_response: PokemonResponse = test::read_body_json(response).await;
    assert_eq!(pokemon_response.name, "pikachu");

    println!("pokemon description {}", pokemon_response.description);
}