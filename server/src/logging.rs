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

    #[derive(Clone)]
    pub(crate) struct Capture {
        buffer: Arc<Mutex<Vec<u8>>>,
        // Keep a second dispatch registered so tracing-core 0.1.32 does not
        // take its single-dispatch callsite cache path for scoped captures.
        // https://github.com/tokio-rs/tracing/issues/2874
        _dispatch_keepalive: Arc<tracing::Dispatch>,
    }

    impl Capture {
        fn new() -> Self {
            Self {
                buffer: Arc::new(Mutex::new(Vec::new())),
                _dispatch_keepalive: Arc::new(tracing::Dispatch::new(
                    tracing::subscriber::NoSubscriber::new(),
                )),
            }
        }
    }

    impl Write for Capture {
        fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
            self.buffer.lock().unwrap().extend_from_slice(bytes);
            Ok(bytes.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    impl Capture {
        pub(crate) fn subscriber(directives: &str) -> (Self, tracing::Dispatch) {
            let capture = Self::new();
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
            String::from_utf8(self.buffer.lock().unwrap().clone()).unwrap()
        }
    }

    #[test]
    fn scoped_subscriber_captures_callsite_first_seen_without_subscriber() {
        const CHILD_ENV: &str = "RPF_SCOPED_CAPTURE_TEST_CHILD";
        // A fresh process prevents other tests from masking the single-dispatch bug.
        if std::env::var_os(CHILD_ENV).is_none() {
            let output = std::process::Command::new(std::env::current_exe().unwrap())
                .args([
                    "logging::tests::scoped_subscriber_captures_callsite_first_seen_without_subscriber",
                    "--exact",
                    "--nocapture",
                ])
                .env(CHILD_ENV, "1")
                .output()
                .unwrap();
            let stdout = String::from_utf8_lossy(&output.stdout);
            assert!(
                output.status.success() && stdout.contains("running 1 test"),
                "child test failed:\n{stdout}\n{}",
                String::from_utf8_lossy(&output.stderr),
            );
            return;
        }

        fn emit() {
            tracing::error!("cold callsite marker");
        }

        let (capture, subscriber) = Capture::subscriber("error");
        emit();
        assert!(capture.text().is_empty());
        tracing::dispatcher::with_default(&subscriber, emit);
        let output = capture.text();
        assert_eq!(output.lines().count(), 1, "{output:?}");
        assert!(output.contains("cold callsite marker"));
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
