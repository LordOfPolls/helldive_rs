use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum HelldiversError {
    RequestError(reqwest::Error),
    JsonError(serde_json::Error),
}

impl fmt::Display for HelldiversError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HelldiversError::RequestError(err) => write!(f, "Request error: {}", err),
            HelldiversError::JsonError(err) => write!(f, "JSON deserialization error: {}", err),
        }
    }
}

impl Error for HelldiversError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            HelldiversError::RequestError(err) => Some(err),
            HelldiversError::JsonError(err) => Some(err),
        }
    }
}

impl From<reqwest::Error> for HelldiversError {
    fn from(err: reqwest::Error) -> Self {
        HelldiversError::RequestError(err)
    }
}

impl From<serde_json::Error> for HelldiversError {
    fn from(err: serde_json::Error) -> Self {
        HelldiversError::JsonError(err)
    }
}