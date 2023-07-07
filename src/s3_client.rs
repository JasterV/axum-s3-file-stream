use aws_sdk_s3::{config::Region, error::SdkError, operation::get_object::GetObjectError, Client};
use tokio::io::AsyncRead;
use tokio_util::io::ReaderStream;

pub struct S3Client {
    client: Client,
}

impl S3Client {
    pub async fn new(region: String, endpoint: String) -> Self {
        let config = aws_config::from_env()
            .region(Region::new(region))
            .endpoint_url(endpoint)
            .load()
            .await;

        // `force_path_style = true` to connect via the `domain.amazonaws.com/bucket` endpoint
        // rather than the Endpoints 2.0 API `bucket.domain.amazonaws.com`
        let builder = aws_sdk_s3::config::Builder::from(&config)
            .force_path_style(true)
            .build();

        Self {
            client: Client::from_conf(builder),
        }
    }

    pub async fn get_object(
        &self,
        bucket: &str,
        key: &str,
    ) -> Result<ReaderStream<impl AsyncRead>, SdkError<GetObjectError>> {
        let object = self
            .client
            .get_object()
            .bucket(bucket)
            .key(key)
            .send()
            .await?;

        let async_read = object.body.into_async_read();
        let stream = ReaderStream::new(async_read);
        Ok(stream)
    }
}
