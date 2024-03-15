use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Window {
    #[serde(default = "Window::default_title")]
    pub title: String,

    #[serde(default = "Window::default_scroll_bar")]
    pub scroll_bar: bool,

    #[serde(default = "Window::default_size")]
    pub size: Size,

    #[serde(default = "Window::default_padding")]
    pub padding: Padding,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Size {
    #[serde(default = "Size::default_width")]
    pub width: u32,

    #[serde(default = "Size::default_height")]
    pub height: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Padding {
    #[serde(default = "Padding::default_horizontal")]
    pub horizontal: u32,

    #[serde(default = "Padding::default_vertical")]
    pub vertical: u32,
}

impl Window {
    fn default_title() -> String {
        "Galactic".to_string()
    }

    fn default_scroll_bar() -> bool {
        true
    }

    fn default_size() -> Size {
        Size::default()
    }

    fn default_padding() -> Padding {
        Padding::default()
    }
}

impl Default for Window {
    fn default() -> Self {
        Self {
            title: Self::default_title(),
            scroll_bar: Self::default_scroll_bar(),
            size: Self::default_size(),
            padding: Self::default_padding(),
        }
    }
}

impl Size {
    fn default_width() -> u32 {
        1200
    }

    fn default_height() -> u32 {
        800
    }
}

impl Default for Size {
    fn default() -> Self {
        Self {
            width: Self::default_width(),
            height: Self::default_height(),
        }
    }
}

impl Padding {
    fn default_horizontal() -> u32 {
        8
    }

    fn default_vertical() -> u32 {
        8
    }
}

impl Default for Padding {
    fn default() -> Self {
        Self {
            horizontal: Self::default_horizontal(),
            vertical: Self::default_vertical(),
        }
    }
}
