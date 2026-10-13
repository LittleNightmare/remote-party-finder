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

#[cfg(test)]
mod tests {
    use super::Config;

    #[test]
    fn preserves_existing_config_format() {
        let config: Config = toml::from_str(r#"
            [web]
            host = "127.0.0.1:8000"
            [mongo]
            url = "mongodb://example.invalid:27017/?authSource=admin"
        "#).unwrap();
        assert_eq!(config.web.host.to_string(), "127.0.0.1:8000");
        assert_eq!(config.mongo.url, "mongodb://example.invalid:27017/?authSource=admin");
    }
}
