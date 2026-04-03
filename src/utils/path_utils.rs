use std::fs;

use crate::common::Config;

pub fn get_config() {
    let mut config_path = dirs::home_dir().expect("Could not find home directory");

    // 2. Build the full path: $HOME/.config/slack/config.toml
    config_path.push(".config");
    config_path.push("slack");
    config_path.push("config.toml");

    // 3. Read the file
    let contents = fs::read_to_string(&config_path).expect("Could not read slack config file");

    // 4. Parse TOML
    let config: Config = toml::from_str(&contents).expect("Failed to parse Slack TOML");

    println!("{:?}", config);
}
