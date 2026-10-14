use std::io::IsTerminal;

use anyhow::{Context, Result};
use tracing_subscriber::filter::{EnvFilter, LevelFilter};

use crate::config::Logging;

fn filter(configured: &str, environment: Option<&str>) -> Result<EnvFilter> {
    let (value, source) = match environment {
        Some(value) => (value, "RUST_LOG"),
        None => (configured, "logging.filter"),
    };
    anyhow::ensure!(!value.trim().is_empty(), "{source} must not be empty");
    EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .with_regex(false)
        .parse(value)
        .with_context(|| format!("invalid {source}"))
}

pub fn init(config: &Logging) -> Result<()> {
    let environment = match std::env::var("RUST_LOG") {
        Ok(value) => Some(value),
        Err(std::env::VarError::NotPresent) => None,
        Err(error) => return Err(error).context("could not read RUST_LOG"),
    };
    let filter = filter(&config.filter, environment.as_deref())?;
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_writer(std::io::stderr)
        .with_ansi(std::io::stderr().is_terminal())
        .try_init()
        .map_err(|error| anyhow::anyhow!("could not initialize logging: {error}"))
}

// Configuration and filter failures happen before the configured subscriber exists.
pub fn report_startup_error(error: &anyhow::Error) {
    let _ = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::ERROR)
        .with_writer(std::io::stderr)
        .with_ansi(false)
        .try_init();
    tracing::error!(error = %format_args!("{error:#}"), "server startup failed");
}

#[cfg(test)]
pub(crate) mod tests {
    use std::io::{self, Write};
    use std::sync::{Arc, Mutex};

    use super::*;

    #[derive(Clone, Default)]
    pub(crate) struct Capture(Arc<Mutex<Vec<u8>>>);

    impl Write for Capture {
        fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
            self.0.lock().unwrap().extend_from_slice(bytes);
            Ok(bytes.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    impl Capture {
        pub(crate) fn subscriber(directives: &str) -> (Self, tracing::Dispatch) {
            let capture = Self::default();
            let writer = capture.clone();
            let subscriber = tracing_subscriber::fmt()
                .with_env_filter(filter(directives, None).unwrap())
                .with_writer(move || writer.clone())
                .with_ansi(false)
                .without_time()
                .finish();
            (capture, tracing::Dispatch::new(subscriber))
        }

        pub(crate) fn text(&self) -> String {
            String::from_utf8(self.0.lock().unwrap().clone()).unwrap()
        }
    }

    #[test]
    fn environment_overrides_configured_filter() {
        let selected = filter("invalid=not-a-level", Some("error")).unwrap();
        assert_eq!(selected.to_string(), "error");
        assert_eq!(filter("debug", None).unwrap().to_string(), "debug");
    }

    #[test]
    fn rejects_invalid_and_empty_filters() {
        for value in ["", "  ", "invalid=not-a-level"] {
            assert!(filter(value, None).is_err(), "{value:?}");
            assert!(filter("info", Some(value)).is_err(), "{value:?}");
        }
    }

    #[test]
    fn levels_and_target_filters_control_real_output() {
        for (directives, debug_enabled, trace_enabled, world_enabled) in [
            ("info", false, false, false),
            ("debug", true, false, true),
            ("trace", true, true, true),
            (
                "debug,remote_party_finder::world_validation=off",
                true,
                false,
                false,
            ),
        ] {
            let (capture, subscriber) = Capture::subscriber(directives);
            tracing::dispatcher::with_default(&subscriber, || {
                tracing::error!("error marker");
                tracing::warn!("warning marker");
                tracing::info!("info marker");
                tracing::debug!("debug marker");
                tracing::trace!("trace marker");
                tracing::debug!(target: "remote_party_finder::world_validation", "world marker");
            });
            let text = capture.text();
            assert!(text.contains("error marker"));
            assert!(text.contains("warning marker"));
            assert!(text.contains("info marker"));
            assert_eq!(text.contains("debug marker"), debug_enabled);
            assert_eq!(text.contains("trace marker"), trace_enabled);
            assert_eq!(text.contains("world marker"), world_enabled);
        }
    }
}
