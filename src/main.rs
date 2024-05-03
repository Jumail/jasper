use std::fs;

use anyhow::Error;
use clap::Parser;
use config::SharedConfig;

mod app;
mod config;
mod conversation;
mod dispatcher;
mod module_manager;
mod modules;
mod types;
mod utils;

fn init_config(config_path: &str) -> Result<SharedConfig, Error> {
    let config_buf = fs::read(config_path)?;
    let config_json_str = String::from_utf8(config_buf)?;
    let config = serde_json::from_str(&config_json_str)?;
    Ok(SharedConfig::new(config))
}

#[derive(Parser)]
struct Args {
    #[arg(short = 'c', long = "config", default_value = "config.json")]
    pub config_path: String,
}

#[tokio::main]
async fn main() {
    // Start logging system.
    if std::env::var(env_logger::DEFAULT_FILTER_ENV).is_ok() {
        pretty_env_logger::init();
    } else {
        // No `RUST_LOG` environment variable found, use `Info` level as default.
        pretty_env_logger::formatted_timed_builder()
            .filter_level(log::LevelFilter::Info)
            .init();
    }

    // Parse command line arguments.
    let args = Args::parse();

    let config = match init_config(&args.config_path) {
        Ok(config) => config,
        Err(err) => {
            log::error!("Failed to load config: {}", err);
            log::error!("Cannot start the bot without a configuration file. exiting...");
            return;
        }
    };

    log::debug!("config success: {:#?}", config);
}
