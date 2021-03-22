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

Once the application is running, you can invoke it via: `http http://localhost:5000/pokemon/charizard`

# Tests

The test suite can be run via: `cargo test`. This will run all tests except for one integration test (`test_pokemon_found`). This test is ignored by default because it hits the real [Shakespearean translation API](https://funtranslations.com/api/shakespeare). Doing this too many times might rate limit your computer, so you might want to do it sparingly. If you do decide to run this integration test, you can do so via `cargo test -- --ignored`.

# Architecture

The project is broken into two crates:

1. `pokemon_core`: This contains the core logic of talking to the API endpoints. The crate is unaware that it is being accessed via a web interface. Having this as a separate crate makes sense because it could be included in an other project with a different, for example command line interface.
2. `server`: This crate contains the entry point of the application and starts an HTTP server that talks to `pokemon_core`.

# Areas of improvement

1. If the Pokemon can not be found, HTTP status code 404 is returned along with the message "Pokemon not found.". All other error cases are handled by returning status code 500 along with a generic error message. This could be improved by adding more specific error messages to certain cases. For example, if the request fails due to rate limiting, this should be communicated to the user of the API, so they will know not to retry unnecessarily.
2. The project uses hard-coded URLs to make requests towards the API endpoints. This should be passed in as configuration instead. This would be useful for integration testing too: if the API endpoints were configurable, then we could spin up a test server which would mock the behaviour of the real API endpoints with the benefit of not having to worry about rate limiting.
3. `.clone()` was used at a couple of places to satisfy the borrow checker. Given more time it might be possible to do away with these.
4. In a production application, one would use a logging library to log errors or informational messages into a file.
5. There are three modules in `pokemon_core` which make network requests: `species_url_request`, `species_description_request` and `shakespearean_translation_request`. We need a way to perform real network requests when running in production and to return mock responses when running unit tests. To achieve this, I defined the networking logic as an external dependency that is passed into the three networking modules.  
I was hoping that this dependency can be defined as a simple callback function. However, I was running into problems when trying to define a type alias to an `async` callback function, so I ended up introducting the `async-trait` crate as a dependency. This allowed me to define the `pokemon_core::JsonRequest` trait such that an easy mock implementation for it can be provided in unit tests. I wish I found a way of doing this without the introduction of a new trait and the `async-trait` crate by relying on core language features only.
6. The Dockerfile could be improved such that pulling the dependencies is done in a separate layer from the layer where the application is built. This would ensure that dependencies are not rebuilt if only the application code changes, thereby making subsequent Docker image builds faster.
7. If the Pokemons do not change too often, one might also want to cache the intermediate results returned from the utilised API methods. This should help with performance and rate limiting concerns.