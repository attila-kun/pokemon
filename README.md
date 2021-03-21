# What is this?

A REST API that, given a Pokemon name, returns its Shakespearean description.

# Usage

The project was developed with Rust version 1.50.0 (stable) on Ubuntu 18.04.5 LTS. To run it, please follow these steps:

## Without Docker:

1. `git clone git@github.com:attila-kun/pokemon.git`
2. `cd pokemon`
3. `cargo run --release`

## With Docker:

1. `git clone git@github.com:attila-kun/pokemon.git`
2. `cd pokemon`
3. `sudo docker build -t pokemon -f ./Dockerfile .`
4. `sudo docker run -p 5000:5000 pokemon:latest`

# Tests

The test suite can be run via: `cargo test`. This will run all tests except for one integration test (`test_pokemon_found`). This test is ignored by default because it hits the real [Shakespearean translation API](https://funtranslations.com/api/shakespeare). Doing this too many times might rate limit your computer, so you might want to do it sparingly. If you do decide to run this integration test, you can do so via `cargo test -- --ignored`.

# Areas of improvement

1. The returned HTTP status codes should be more sophisticated. Right now either status code 200 is returned in case of a successful response, or 500 is returned in case something went wrong. For example when the Pokemon does not exist, status code 404 could be returned. Similar improvements can be made to the error messages returned to the user: instead of using a generic error message, we could display a more specific reason.
2. The project uses hard-coded URLs to make requests towards the API endpoints. This should be passed in as configuration instead. This might be useful for integration testing too: if the API endpoints are configurable, then we could spin up a test server which would mock the behaviour of the real API endpoints with the benefit of not having to worry about rate limiting.
3. `.clone()` was used at a couple of places to satisfy the borrow checker. Given more time it might be possible to do away with these.
4. In a production application, one would use a logging library to log errors or informational messages into a file.
5. The Dockerfile could be improved such that pulling the dependencies is done in a separate layer from the layer where the application is built. This would ensure that dependencies would not get rebuilt if only the application code changes, thereby making subsequent Docker image builds faster.