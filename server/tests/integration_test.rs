use actix_web::{test, App};
use server::{config, PokemonResponse};

#[actix_rt::test]
// Test is ignored because it hits the real Shakespearean translator API. If called too many times, then the API's rate limiting
// would kick in and the computer running the app would not be able to use the translation API for a certain period.
#[ignore]
async fn test_pokemon_found() {
    let mut app = test::init_service(App::new().configure(config)).await;
    let request = test::TestRequest::with_uri("/pokemon/pikachu").to_request();
    let response = test::call_service(&mut app, request).await;

    assert!(response.status().is_success());

    let pokemon_response: PokemonResponse = test::read_body_json(response).await;
    assert_eq!(pokemon_response.name, "pikachu");
    assert_eq!(pokemon_response.description, "'t stores electricity in the electric sacs on its cheeks. At which hour 't releases pent-up energy in a did burst,  the electric power is equal to a lightning bolt.");
}

#[actix_rt::test]
async fn test_index_pokemon_not_found() {
    let mut app = test::init_service(App::new().configure(config)).await;
    let request = test::TestRequest::with_uri("/pokemon/rocinante").to_request();
    let response = test::call_service(&mut app, request).await;

    assert!(response.status().is_client_error());
}