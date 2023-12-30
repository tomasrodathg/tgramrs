use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub api_id: i32,
    pub api_hash: &'static str,
}

impl AppConfig {
    pub fn new(api_id: i32, api_hash: &'static str) -> Self {
        Self { api_id, api_hash }
    }
}
