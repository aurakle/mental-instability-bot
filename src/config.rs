use std::sync::OnceLock;

use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub token: String,
    pub quotes_channel: Option<u64>,
    pub log_extensions: Option<Vec<String>>,
    pub db_username: String,
    pub db_password: String,
    pub db_host: String,
}

pub fn get_config() -> &'static Config {
    static CONFIG: OnceLock<Config> = OnceLock::new();
    CONFIG.get_or_init(|| {
        println!("Reading config...");
        toml::from_str(&fs::read_to_string("config.toml")
            .expect("Cannot read config.toml"))
            .expect("Failed to parse config.toml")
    })
}
