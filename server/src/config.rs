use std::net::SocketAddr;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub web: Web,
    pub mongo: Mongo,
}

#[derive(Deserialize)]
pub struct Web {
    pub host: SocketAddr,
}

#[derive(Deserialize)]
pub struct Mongo {
    pub url: String,
}
