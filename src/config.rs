use serde::{Deserialize, Serialize};
use std::fs;
use std::io::prelude::*;
use std::path::Path;

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub prefixes: Vec<String>,
}

impl Config {
    pub fn from_file(file_path: &str) -> Config {
        if Path::new(file_path).exists() {
            let config_str = fs::read_to_string(file_path).expect("Failed to read config file");
            serde_json::from_str(&config_str).expect("Failed to parse config file")
        } else {
            Self::create_default_config(file_path)
        }
    }

    pub fn create_default_config(file_path: &str) -> Config {
        let default_config = Config {
            prefixes: vec![
                "[FEATURE]".to_string(),
                "[BUGFIX]".to_string(),
                "[BUILD]".to_string(),
                "[STYLE]".to_string(),
                "[REFACTOR]".to_string(),
                "[DOCS]".to_string(),
                "[TEST]".to_string(),
            ],
        };

        let json = serde_json::to_string_pretty(&default_config)
            .expect("Failed to serialize the config to JSON");
        if let Some(parent) = Path::new(file_path).parent() {
            fs::create_dir_all(parent).expect("Failed to create config directory");
        }

        let mut file = fs::File::create(file_path).expect("Failed to create config file");
        file.write_all(json.as_bytes())
            .expect("Failed to write to config file");

        default_config
    }
}
