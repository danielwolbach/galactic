use gtk::glib;
use std::{fmt::Display, io};
use toml::de;

pub type Result<T> = std::result::Result<T, Error>;

pub enum Error {
    Io(io::Error),
    Deserialize(de::Error),
    Rgba(glib::BoolError),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Io(error) => write!(f, "{error}"),
            Error::Deserialize(error) => write!(f, "{error}"),
            Error::Rgba(error) => write!(f, "{error}"),
        }
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}

impl From<de::Error> for Error {
    fn from(error: de::Error) -> Self {
        Self::Deserialize(error)
    }
}

impl From<glib::BoolError> for Error {
    fn from(error: glib::BoolError) -> Self {
        Self::Rgba(error)
    }
}
