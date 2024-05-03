use std::collections::HashSet;
use std::sync::Arc;

use serde::Deserialize;

/// A thread-safe reference-counting object that represents
/// a [`Config`] instance.
#[derive(Debug, Clone)]
pub struct SharedConfig {
    pub config: Arc<Config>,
}

impl SharedConfig {
    /// Constructs a new `SharedConfig`.
    pub fn new(config: Config) -> Self {
        Self {
            config: Arc::new(config),
        }
    }
}

/// Top-level config type fot the bot.
#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    /// Telegram bot token
    /// JSON key: `botToken`
    #[serde(rename = "botToken")]
    pub telegram_bot_token: String,
    /// Database URL
    /// JSON key: `databaseUrl`
    #[serde(rename = "databaseUrl")]
    pub database_url: String,
    /// A set of usernames that represents the admin users, who can use
    /// admin commands. You must specify this field to use admin features.
    /// JSON key: `adminUsernames`
    #[serde(default, rename = "adminUsernames")]
    pub admin_usernames: HashSet<String>,
    // A set of usernames that are allowed to use the bot.
    // You must specify this field to use the bot.
    // JSON key: `whitelistedUsernames`
    #[serde(default, rename = "whitelistedUsernames")]
    pub whitelisted_usernames: HashSet<String>,
    // Ollama API URL
    // JSON key: `ollamaApiUrl`
    #[serde(default, rename = "ollamaApiUrl")]
    pub ollama_api_url: String,
}
