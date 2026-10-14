use std::net::SocketAddr;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub web: Web,
    pub mongo: Mongo,
    #[serde(default)]
    pub validation: Validation,
    #[serde(default)]
    pub logging: Logging,
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct Logging {
    pub filter: String,
}

impl Default for Logging {
    fn default() -> Self {
        Self { filter: "info".into() }
    }
}

#[derive(Deserialize)]
pub struct Web {
    pub host: SocketAddr,
}

#[derive(Deserialize)]
pub struct Mongo {
    pub url: String,
}

#[derive(Deserialize, Default)]
pub struct Validation {
    #[serde(default)]
    pub created_world_id_ranges: Vec<[u16; 2]>,
}

impl Validation {
    pub fn allows_created_world_id(&self, world_id: u16) -> bool {
        self.created_world_id_ranges.is_empty()
            || self
                .created_world_id_ranges
                .iter()
                .any(|&[start, end]| (start..=end).contains(&world_id))
    }
}

#[cfg(test)]
mod tests {
    use super::{Config, Logging};

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
        assert!(config.validation.created_world_id_ranges.is_empty());
        assert_eq!(config.logging.filter, "info");
    }

    #[test]
    fn parses_created_world_id_ranges() {
        let config: Config = toml::from_str(r#"
            [web]
            host = "127.0.0.1:8000"
            [mongo]
            url = "mongodb://example.invalid:27017"
            [validation]
            created_world_id_ranges = [[1000, 1999], [4000, 4999]]
        "#).unwrap();

        assert_eq!(config.validation.created_world_id_ranges, vec![[1000, 1999], [4000, 4999]]);
        assert!(config.validation.allows_created_world_id(1000));
        assert!(config.validation.allows_created_world_id(4999));
        assert!(!config.validation.allows_created_world_id(999));
        assert!(!config.validation.allows_created_world_id(3000));
    }

    #[test]
    fn empty_created_world_id_ranges_allow_every_world() {
        let validation: super::Validation = toml::from_str(
            "created_world_id_ranges = []"
        ).unwrap();

        assert!(validation.allows_created_world_id(0));
        assert!(validation.allows_created_world_id(u16::MAX));
    }

    #[test]
    fn parses_logging_filter_without_affecting_validation() {
        let config: Config = toml::from_str(r#"
            [web]
            host = "127.0.0.1:8000"
            [mongo]
            url = "mongodb://example.invalid:27017"
            [validation]
            created_world_id_ranges = [[1000, 1999], [4000, 4999]]
            [logging]
            filter = "warn,remote_party_finder=debug"
        "#).unwrap();

        assert_eq!(config.logging.filter, "warn,remote_party_finder=debug");
        assert_eq!(config.validation.created_world_id_ranges, vec![[1000, 1999], [4000, 4999]]);
        assert!(config.validation.allows_created_world_id(1000));
        assert!(!config.validation.allows_created_world_id(3000));
    }

    #[test]
    fn logging_filter_defaults_to_info() {
        let logging: Logging = toml::from_str("").unwrap();

        assert_eq!(logging.filter, "info");
    }
}
