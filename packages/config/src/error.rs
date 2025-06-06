use std::io;

impl From<io::Error> for ConfigError {
    fn from(err: io::Error) -> Self {
        ConfigError::IoError(err)
    }
}

impl From<serde_json::Error> for ConfigError {
    fn from(err: serde_json::Error) -> Self {
        ConfigError::ParseError(ParseError::Json(err))
    }
}

impl From<serde_yaml::Error> for ConfigError {
    fn from(err: serde_yaml::Error) -> Self {
        ConfigError::ParseError(ParseError::Yaml(err))
    }
}

#[derive(Debug)]
pub enum ParseError {
    Json(serde_json::Error),
    Yaml(serde_yaml::Error),
}

#[derive(Debug)]
pub enum ConfigError {
    IoError(std::io::Error),
    ParseError(ParseError),
    InvalidConfig(String),
    ConfigNotFound,
}
