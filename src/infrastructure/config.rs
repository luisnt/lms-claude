use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AppConfig {
    pub selected_model: Option<String>,
}

impl AppConfig {
    pub fn load() -> Self {
        if let Some(path) = Self::get_path() {
            if path.exists() {
                if let Ok(content) = fs::read_to_string(&path) {
                    if let Ok(config) = serde_json::from_str(&content) {
                        return config;
                    }
                }
            }
        }
        Self::default()
    }

    pub fn save(&self) -> Result<()> {
        if let Some(path) = Self::get_path() {
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent)?;
            }
            let json = serde_json::to_string_pretty(self)?;
            fs::write(&path, json)?;
        }
        Ok(())
    }

    fn get_path() -> Option<PathBuf> {
        dirs::config_dir().map(|mut path| {
            path.push("lms-claude");
            path.push("config.json");
            path
        })
    }
}
