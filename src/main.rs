mod configuration;
mod error;
mod s3_client;

use aws_sdk_s3::primitives::ByteStreamError;
use aws_sdk_s3::Client as S3Client;
use axum::{
    body::StreamBody,
    extract::{Path, State},
    routing::get,
    Router,
};
use bytes::Bytes;
use configuration::Configuration;
use error::AppError;
use futures_util::Stream;
use std::sync::Arc;

pub struct AppState {
    pub config: Configuration,
    pub s3_client: S3Client,
}

async fn download(
    State(state): State<Arc<AppState>>,
    Path(file_name): Path<String>,
) -> Result<StreamBody<impl Stream<Item = Result<Bytes, ByteStreamError>>>, AppError> {
    let object = state
        .s3_client
        .get_object()
        .bucket(state.config.bucket.as_ref())
        .key(file_name)
        .send()
        .await?;

    Ok(StreamBody::new(object.body))
}

fn router(state: AppState) -> Router {
    Router::new()
        .route("/:file", get(download))
        .with_state(Arc::new(state))
}

#[tokio::main]
async fn main() {
    let config = Configuration::load().expect("Error loading configuration");
    let s3_client = s3_client::build(&config.aws_s3.region, &config.aws_s3.endpoint).await;
    let address = config.address();
    let app_state = AppState { config, s3_client };

    axum::Server::bind(&address)
        .serve(router(app_state).into_make_service())
        .await
        .unwrap();
}
