use self::{error::Result, general::General, window::Window};
use serde::{Deserialize, Serialize};
use std::{fmt::Debug, path::Path};

pub mod error;
pub mod general;
pub mod window;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    #[serde(default = "Config::default_general")]
    pub general: General,

    #[serde(default = "Config::default_window")]
    pub window: Window,
}

impl Config {
    pub fn load<P>(path: P) -> Result<Self>
    where
        P: AsRef<Path> + Debug,
    {
        tracing::info!("Loading config from {path:?}.");
        let file_contents = std::fs::read_to_string(path)?;
        Ok(toml::from_str(&file_contents)?)
    }

    fn default_general() -> General {
        General::default()
    }

    fn default_window() -> Window {
        Window::default()
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            general: Self::default_general(),
            window: Self::default_window(),
        }
    }
}
