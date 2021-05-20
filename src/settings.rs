use config::{Config, ConfigError, Environment};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Settings {
    pub bot_token: String,
    pub api_token: String,
    pub log_level: log::LevelFilter,
}

impl Settings {
    pub fn read() -> Result<Settings, ConfigError> {
        let mut cfg = Config::new();
        cfg.merge(Environment::with_prefix("pics_bot"))?;
        cfg.try_into()
    }
}
