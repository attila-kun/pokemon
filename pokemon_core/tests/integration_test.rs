use pokemon_core::get_shakespearean_description;

#[actix_rt::test]
async fn test_index_get() {
    println!("{:?}", get_shakespearean_description("charizard").await);
}