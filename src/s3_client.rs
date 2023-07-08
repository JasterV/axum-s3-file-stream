use aws_sdk_s3::{config::Region, Client};

pub async fn build(region: &str, endpoint: &str) -> Client {
    let config = aws_config::from_env()
        .region(Region::new(region.to_owned()))
        .endpoint_url(endpoint)
        .load()
        .await;

    // `force_path_style = true` to connect via the `domain.amazonaws.com/bucket` endpoint
    // rather than the Endpoints 2.0 API `bucket.domain.amazonaws.com`
    let builder = aws_sdk_s3::config::Builder::from(&config)
        .force_path_style(true)
        .build();

    Client::from_conf(builder)
}
