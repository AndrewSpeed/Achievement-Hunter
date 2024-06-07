use config::{Config, ConfigError, File};
use serde::Deserialize;
use std::path::Path;

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
    pub fn new(path: &Path) -> Result<Self, ConfigError> {
        let builder = Config::builder();

        builder
            .add_source(File::with_name(path.to_str().unwrap()))
            .build()?
            .try_deserialize()
    }
}
