use aws_sdk_s3::{error::SdkError, operation::get_object::GetObjectError};
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

// Make our own error that wraps `anyhow::Error`.
#[derive(Debug)]
pub struct AppError(anyhow::Error);

// Tell axum how to convert `AppError` into a response.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self.0.downcast_ref::<SdkError<GetObjectError>>() {
            Some(SdkError::ServiceError(service_error)) if service_error.err().is_no_such_key() => {
                StatusCode::NOT_FOUND.into_response()
            }
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Internal error: {self:#?}"),
            )
                .into_response(),
        }
    }
}

// This enables using `?` on functions that return `Result<_, anyhow::Error>` to turn them into
// `Result<_, AppError>`. That way you don't need to do that manually.
impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
