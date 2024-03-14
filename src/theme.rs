use crate::{config::Config, error::Result};
use serde::Deserialize;
use std::path::{Path, PathBuf};

#[derive(Debug, Deserialize)]
pub struct Theme {
    pub foreground: String,
    pub background: String,
    pub palette: Vec<String>,
}

impl Theme {
    pub fn load_toml<P: AsRef<Path>>(path: P) -> Result<Self> {
        let contents = std::fs::read_to_string(Self::directory().join(path))?;
        Ok(toml::from_str(&contents)?)
    }

    pub fn directory() -> PathBuf {
        Config::directory().join("themes")
    }
}

impl Default for Theme {
    fn default() -> Self {
        let foreground = "#ffffff".to_string();
        let background = "#1e1e1e".to_string();
        let palette = [
            "#241F31", "#C01C28", "#2EC27E", "#F5C211", "#1E78E4", "#9841BB", "#0AB9DC", "#C0BFBC",
            "#5E5C64", "#ED333B", "#57E389", "#F8E45C", "#51A1FF", "#C061CB", "#4FD2FD", "#F6F5F4",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();

        Self {
            foreground,
            background,
            palette,
        }
    }
}
