use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct General {
    #[serde(default = "General::default_command")]
    pub command: Vec<String>,

    #[serde(default = "General::default_font")]
    pub font: String,

    #[serde(default = "General::default_theme")]
    pub theme: Option<String>,
}

impl General {
    fn default_command() -> Vec<String> {
        ["/usr/bin/bash".to_string()].to_vec()
    }

    fn default_font() -> String {
        "Monospace 12".to_string()
    }

    fn default_theme() -> Option<String> {
        None
    }
}

impl Default for General {
    fn default() -> Self {
        Self {
            command: Self::default_command(),
            font: Self::default_font(),
            theme: Self::default_theme(),
        }
    }
}
