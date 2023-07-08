use serde::Deserialize;
use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    sync::Arc,
};

#[derive(Deserialize)]
pub struct Configuration {
    pub port: u16,
    pub bucket: Arc<str>,
    pub aws_s3: AwsS3,
}

#[derive(Clone, Deserialize)]
pub struct AwsS3 {
    pub endpoint: Arc<str>,
    pub region: Arc<str>,
}

impl Configuration {
    pub fn address(&self) -> SocketAddr {
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), self.port)
    }
}
