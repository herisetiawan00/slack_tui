use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_url: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            client_id: Default::default(),
            client_secret: Default::default(),
            redirect_url: String::from("https://127.0.0.1:7777"),
        }
    }
}
