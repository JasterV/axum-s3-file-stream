mod configuration;

use axum::{response::IntoResponse, routing::get, Router};
use config::Config;
use configuration::Configuration;

async fn hello_world() -> impl IntoResponse {
    "Hello, World!"
}

fn router() -> Router {
    Router::new().route("/", get(hello_world))
}

#[tokio::main]
async fn main() {
    let config: Configuration = Config::builder()
        .add_source(
            config::Environment::default()
                .try_parsing(true)
                .separator("__"),
        )
        .build()
        .expect("Failed to load app configuration")
        .try_deserialize()
        .expect("Cannot deserialize configuration");

    // run it with hyper on localhost:3000
    axum::Server::bind(&config.address())
        .serve(router().into_make_service())
        .await
        .unwrap();
}
