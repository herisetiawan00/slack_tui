use std::{env, fs, path::PathBuf, process::Command};

use crate::{
    common::{Config, Injectable},
    data::datasources::local::LocalDatasource,
};

pub struct ConfigurationLocalDatasource {}

impl Injectable for ConfigurationLocalDatasource {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl LocalDatasource for ConfigurationLocalDatasource {}

impl ConfigurationLocalDatasource {
    pub fn get_path() -> PathBuf {
        let mut config_path = dirs::home_dir().expect("Could not find home directory");
        config_path.push(".config");
        config_path.push("slack");

        fs::create_dir_all(&config_path)
            .expect(format!("Could not initialize config folder at: {:?}", config_path).as_str());

        config_path.push("config.toml");
        return config_path;
    }

    pub fn initialize(&self) -> Config {
        let config = Config::default();

        let path = Self::get_path();

        let content = toml::to_string(&config).expect("Could not serialize config");

        fs::write(&path, content)
            .expect(format!("Could not initialize config at: {:?}", path).as_str());

        return config;
    }

    pub fn get(&self) -> Config {
        let path = Self::get_path();

        if let Ok(contents) = fs::read_to_string(&path) {
            let config: Config = toml::from_str(&contents).expect("Failed to parse Slack TOML");
            return config;
        } else {
            return Self::initialize(self);
        }
    }

    pub fn edit(&self) -> Option<()> {
        let config_path = Self::get_path();
        let editor = env::var("EDITOR").unwrap_or_else(|_| "vim".to_string());
        let mut child = Command::new(editor).arg(config_path).spawn().ok()?;
        child.wait().ok()?;

        return Some(());
    }
}
