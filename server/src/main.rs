#![feature(try_blocks, iter_intersperse)]

use anyhow::Context;
use std::borrow::Cow;
use std::path::Path;
use std::sync::Arc;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use crate::config::Config;

mod config;
mod listing;
mod listing_container;
mod base64_sestring;
mod sestring_ext;
mod stats;
mod web;
mod template;
mod ffxiv;

#[cfg(test)]
mod test;

#[tokio::main]
async fn main() {
    let mut args: Vec<String> = std::env::args().skip(1).collect();
    let config_path = if args.is_empty() {
        Cow::from("./config.toml")
    } else {
        Cow::from(args.remove(0))
    };

    let config = match get_config(&*config_path).await {
        Ok(config) => config,
        Err(e) => {
            eprintln!("error: {}", e);
            return;
        }
    };

    if let Err(e) = self::web::start(Arc::new(config)).await {
        eprintln!("error: {}", e);
        eprintln!("  {:?}", e);
        eprintln!("{}", e.backtrace());
    }
}

async fn get_config<P: AsRef<Path>>(path: P) -> anyhow::Result<Config> {
    let mut f = File::open(path).await.context("could not open config file")?;
    let mut toml = String::new();
    f.read_to_string(&mut toml).await.context("could not read config file")?;
    let config = toml::from_str(&toml).context("could not parse config file")?;

    Ok(config)
}
