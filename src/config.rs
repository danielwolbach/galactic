use crate::error::Result;
use serde::Deserialize;
use std::path::{Path, PathBuf};

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(default = "Config::default_general")]
    pub general: GeneralConfig,

    #[serde(default = "Config::default_window")]
    pub window: WindowConfig,
}

#[derive(Debug, Deserialize)]
pub struct GeneralConfig {
    #[serde(deserialize_with = "crate::utils::deserialize_vec_from_value_toml")]
    #[serde(default = "GeneralConfig::default_command")]
    pub command: Vec<String>,

    #[serde(default = "GeneralConfig::default_font")]
    pub font: String,

    #[serde(default = "GeneralConfig::default_theme")]
    pub theme: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct WindowConfig {
    #[serde(default = "WindowConfig::default_title")]
    pub title: String,

    #[serde(default = "WindowConfig::default_scroll_bar")]
    pub scroll_bar: bool,

    #[serde(default = "WindowConfig::default_size")]
    pub size: WindowSizeConfig,

    #[serde(default = "WindowConfig::default_padding")]
    pub padding: WindowPaddingConfig,
}

#[derive(Debug, Deserialize)]
pub struct WindowSizeConfig {
    #[serde(default = "WindowSizeConfig::default_width")]
    pub width: u32,

    #[serde(default = "WindowSizeConfig::default_height")]
    pub height: u32,
}

#[derive(Debug, Deserialize)]
pub struct WindowPaddingConfig {
    #[serde(default = "WindowPaddingConfig::default_x")]
    pub x: u32,

    #[serde(default = "WindowPaddingConfig::default_y")]
    pub y: u32,
}

impl Config {
    pub fn load_toml<P: AsRef<Path>>(path: P) -> Result<Self> {
        let contents = std::fs::read_to_string(Self::directory().join(path))?;
        Ok(toml::from_str(&contents)?)
    }

    pub fn directory() -> PathBuf {
        dirs::config_dir().unwrap().join("galactic")
    }

    pub fn default_general() -> GeneralConfig {
        GeneralConfig::default()
    }

    pub fn default_window() -> WindowConfig {
        WindowConfig::default()
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

impl GeneralConfig {
    pub fn default_command() -> Vec<String> {
        vec![std::env::var("SHELL").unwrap_or("/usr/bin/bash".to_string())]
    }

    pub fn default_font() -> String {
        "Monospace 12".to_string()
    }

    pub fn default_theme() -> Option<String> {
        None
    }
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            command: Self::default_command(),
            font: Self::default_font(),
            theme: Self::default_theme(),
        }
    }
}

impl WindowConfig {
    pub fn default_title() -> String {
        "Galactic".into()
    }

    pub fn default_scroll_bar() -> bool {
        true
    }

    pub fn default_size() -> WindowSizeConfig {
        WindowSizeConfig::default()
    }

    pub fn default_padding() -> WindowPaddingConfig {
        WindowPaddingConfig::default()
    }
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            title: Self::default_title(),
            scroll_bar: Self::default_scroll_bar(),
            size: Self::default_size(),
            padding: Self::default_padding(),
        }
    }
}

impl WindowSizeConfig {
    pub fn default_width() -> u32 {
        1200
    }

    pub fn default_height() -> u32 {
        800
    }
}

impl Default for WindowSizeConfig {
    fn default() -> Self {
        Self {
            width: Self::default_width(),
            height: Self::default_height(),
        }
    }
}

impl WindowPaddingConfig {
    pub fn default_x() -> u32 {
        8
    }

    pub fn default_y() -> u32 {
        8
    }
}

impl Default for WindowPaddingConfig {
    fn default() -> Self {
        Self {
            x: Self::default_x(),
            y: Self::default_y(),
        }
    }
}
