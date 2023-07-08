mod configuration;
mod s3_client;

use aws_sdk_s3::primitives::ByteStreamError;
use aws_sdk_s3::Client as S3Client;
use axum::{
    body::StreamBody,
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Router,
};
use bytes::Bytes;
use config::Config;
use configuration::Configuration;
use futures_util::Stream;
use std::sync::Arc;

pub struct AppState {
    pub config: Configuration,
    pub s3_client: S3Client,
}

async fn download(
    State(state): State<Arc<AppState>>,
    Path(file_name): Path<Arc<str>>,
) -> Result<StreamBody<impl Stream<Item = Result<Bytes, ByteStreamError>>>, (StatusCode, String)> {
    let object = state
        .s3_client
        .get_object()
        .bucket(state.config.bucket.as_ref())
        .key(file_name.as_ref())
        .send()
        .await
        .map_err(|err| {
            (
                StatusCode::IM_A_TEAPOT,
                format!("There was an error: {err:?}"),
            )
        })?;

    Ok(StreamBody::new(object.body))
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
    let s3_client = s3_client::build(&config.aws_s3.region, &config.aws_s3.endpoint).await;
    let app_state = AppState { config, s3_client };

    axum::Server::bind(&address)
        .serve(router(app_state).into_make_service())
        .await
        .unwrap();
}
