use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Write};
use std::path::Path;

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub prefixes: Vec<String>,
}

impl Config {
    pub fn from_file(file_path: &str) -> io::Result<Config> {
        if Path::new(file_path).exists() {
            let config_str = fs::read_to_string(file_path)?;
            Ok(serde_json::from_str(&config_str).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?)
        } else {
            Self::create_default_config(file_path)
        }
    }

    pub fn create_default_config(file_path: &str) -> io::Result<Config> {
        let default_config = Config {
            prefixes: vec![
                "build".to_string(),
                "chore".to_string(),
                "ci".to_string(),
                "feat".to_string(),
                "fix".to_string(),
                "style".to_string(),
                "refactor".to_string(),
                "docs".to_string(),
                "perf".to_string(),
                "test".to_string(),
                "revert".to_string(),
            ],
        };

        let json = serde_json::to_string_pretty(&default_config)?;
        if let Some(parent) = Path::new(file_path).parent() {
            fs::create_dir_all(parent)?;
        }

        let mut file = fs::File::create(file_path).unwrap();
        file.write(json.as_bytes()).unwrap();

        Ok(default_config)
    }
}
