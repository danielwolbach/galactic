use regex::Regex;
use serde::Deserialize;

pub fn color<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let color = String::deserialize(deserializer)?;
    let regex = Regex::new(r"^#[0-9a-fA-F]{6}$").unwrap();

    tracing::trace!("Validate hex color {color}.");

    if !regex.is_match(&color) {
        return Err(serde::de::Error::custom(format!(
            "Invalid hex color {color}."
        )));
    }

    Ok(color)
}

pub fn color_palette<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    static EXPECTED_PALETTE_SIZE: usize = 16;
    let colors: Vec<String> = Vec::deserialize(deserializer)?;

    tracing::debug!("Deserialize color paette ``{colors:?}``");

    if colors.len() != EXPECTED_PALETTE_SIZE {
        return Err(serde::de::Error::custom(format!(
            "Invalid color palette. Expected are {EXPECTED_PALETTE_SIZE} values but {} were provided.",
            colors.len()
        )));
    }

    let regex = Regex::new(r"^#[0-9a-fA-F]{6}$").unwrap();
    for color in &colors {
        tracing::trace!("Validate hex color {color}.");
        if !regex.is_match(color) {
            return Err(serde::de::Error::custom(format!(
                "Invalid hex color {color}."
            )));
        }
    }

    Ok(colors)
}
