mod configuration;

use std::sync::Arc;

use axum::{
    extract::{Path, State},
    response::IntoResponse,
    routing::get,
    Router,
};
use config::Config;
use configuration::Configuration;
use tokio::fs::File;

async fn download(
    State(config): State<Arc<Configuration>>,
    Path(file_name): Path<String>,
) -> impl IntoResponse {
    let full_path = format!("{}/{}", config.files_path, file_name);
    let file = File::open(full_path).await.unwrap();
    let stream = tokio_util::io::ReaderStream::new(file);
    "hi"
}

async fn hello_world() -> impl IntoResponse {
    "Hello, World!"
}

fn router(config: Configuration) -> Router {
    Router::new()
        .route("/", get(hello_world))
        .with_state(Arc::new(config))
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

    axum::Server::bind(&config.address())
        .serve(router(config).into_make_service())
        .await
        .unwrap();
}
