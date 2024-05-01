use config::{Config, ConfigError, File};
use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize)]
pub struct Steam {
    pub api_key: String,
    pub user_id: String,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub steam: Steam,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut builder = Config::builder();

        if let Ok(home_dir) = env::var("HOME") {
            builder = builder.add_source(File::with_name(&format!(
                "{home_dir}/.config/achievement_hunter/config.toml"
            )));
        }

        builder.build()?.try_deserialize()
    }
}
