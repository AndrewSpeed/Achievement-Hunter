use anyhow::{bail, Error};
use config::{Config, ConfigError, File};
use serde::Deserialize;
use std::env;
use std::path::Path;

const CONFIG_DIR_NAME: &str = "achievement_hunter";
const CONFIG_FILENAME: &str = "config.toml";

#[cfg(not(target_os = "windows"))]
pub fn get_config_filepath() -> Result<String, Error> {
    let home_dir = match env::var("HOME") {
        Ok(var) => var,
        Err(err) => bail!(err),
    };
    let path = Path::new(&home_dir)
        .join(".config")
        .join(CONFIG_DIR_NAME)
        .join(CONFIG_FILENAME);

    Ok(path
        .to_str()
        .expect("Unable to convert config filepath to str")
        .to_string())
}

#[cfg(target_os = "windows")]
pub fn get_config_filepath() -> Result<String, Error> {
    let path = Path::new("C:\\ProgramData\\achievement_hunter")
        .join(".config")
        .join(CONFIG_DIR_NAME)
        .join(CONFIG_FILENAME);

    Ok(path
        .to_str()
        .expect("Unable to convert config filepath to str")
        .to_string())
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct Steam {
    pub api_key: String,
    pub user_id: String,
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
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
