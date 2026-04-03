use std::{fs, path::PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub client_id: String,
    pub client_secret: String,
}

impl Config {
    pub fn default() -> Self {
        Self {
            client_id: String::new(),
            client_secret: String::new(),
        }
    }

    pub fn get_path() -> PathBuf {
        let mut config_path = dirs::home_dir().expect("Could not find home directory");
        config_path.push(".config");
        config_path.push("slack");

        fs::create_dir_all(&config_path).expect(format!("Could not initialize config folder at: {:?}", config_path).as_str());

        config_path.push("config.toml");
        return config_path;
    }

    pub fn initialize() -> Self {
        let config = Self::default();

        let path = Self::get_path();

        let content = toml::to_string(&config).expect("Could not serialize config");

        fs::write(&path, content)
            .expect(format!("Could not initialize config at: {:?}", path).as_str());

        return config;
    }

    pub fn get() -> Self {
        let path = Self::get_path();

        if let Ok(contents) = fs::read_to_string(&path) {
            let config: Config = toml::from_str(&contents).expect("Failed to parse Slack TOML");
            return config;
        } else {
            return Self::initialize();
        }
    }
}
