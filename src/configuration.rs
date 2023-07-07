use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Configuration {
    pub port: u16,
    pub bucket: String,
    pub aws_s3: AwsS3,
}

#[derive(Clone, Deserialize)]
pub struct AwsS3 {
    pub endpoint: String,
    pub region: String,
}

impl Configuration {
    pub fn address(&self) -> SocketAddr {
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), self.port)
    }
}
