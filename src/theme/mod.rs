use self::error::Result;
use serde::{Deserialize, Serialize};
use std::{fmt::Debug, path::Path};

mod deserialize;
mod error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    #[serde(deserialize_with = "deserialize::color")]
    pub foreground: String,

    #[serde(deserialize_with = "deserialize::color")]
    pub background: String,

    #[serde(deserialize_with = "deserialize::color_palette")]
    pub palette: Vec<String>,
}

impl Theme {
    pub fn load<P>(path: P) -> Result<Self>
    where
        P: AsRef<Path> + Debug,
    {
        tracing::info!("Loading theme from {path:?}.");
        let file_contents = std::fs::read_to_string(path)?;
        Ok(toml::from_str(&file_contents)?)
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
