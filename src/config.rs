use std::fs;
use std::path::{Path, PathBuf};

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub template_path: PathBuf,
}

impl Config {
    pub fn new() -> Self {
        let config_path = Path::new(".").join("jsonbuilder.toml");

        if config_path.exists() {
            Self::from_file(&config_path)
        } else {
            Self::default()
        }
    }

    fn from_file(path: &Path) -> Self {
        let config = fs::read_to_string(path).unwrap();
        toml::from_str(&config).unwrap()
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            template_path: "templates".into(),
        }
    }
}
