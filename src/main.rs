mod configuration;
mod s3_client;

use axum::{
    body::StreamBody,
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Router,
};
use config::Config;
use configuration::Configuration;
use s3_client::S3Client;
use std::sync::Arc;
use tokio::io::AsyncRead;
use tokio_util::io::ReaderStream;

pub struct AppState {
    pub config: Configuration,
    pub s3_client: S3Client,
}

async fn download(
    State(state): State<Arc<AppState>>,
    Path(file_name): Path<String>,
) -> Result<StreamBody<ReaderStream<impl AsyncRead>>, (StatusCode, String)> {
    let stream = state
        .s3_client
        .get_object(&state.config.aws_s3.bucket, &file_name)
        .await
        .map_err(|err| {
            (
                StatusCode::IM_A_TEAPOT,
                format!("There was an error: {err:?}"),
            )
        })?;

    Ok(StreamBody::new(stream))
}

fn router(state: AppState) -> Router {
    Router::new()
        .route("/:file", get(download))
        .with_state(Arc::new(state))
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

    let address = config.address();
    let aws_s3_config = config.aws_s3.clone();
    let s3_client = S3Client::new(aws_s3_config.region, aws_s3_config.endpoint).await;
    let app_state = AppState { config, s3_client };

    axum::Server::bind(&address)
        .serve(router(app_state).into_make_service())
        .await
        .unwrap();
}
