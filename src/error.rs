use std::error::Error;
use reqwest::Response;
use reqwest::StatusCode;
use std::fmt;

#[derive(Debug)]
pub enum HelldiversError {
    RequestError(reqwest::Error),
    JsonError(serde_json::Error),
    APIError(String),
    InvalidWarId(String),
}

impl fmt::Display for HelldiversError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HelldiversError::RequestError(err) => write!(f, "Request error: {}", err),
            HelldiversError::JsonError(err) => write!(f, "JSON deserialization error: {}", err),
            HelldiversError::InvalidWarId(message) => write!(f, "Invalid war ID: {}", message),
            HelldiversError::APIError(message) => write!(f, "API error: {}", message),
        }
    }
}

impl Error for HelldiversError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            HelldiversError::RequestError(err) => Some(err),
            HelldiversError::JsonError(err) => Some(err),
            HelldiversError::InvalidWarId(_) => None,
            HelldiversError::APIError(_) => None,
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

impl From<Response> for HelldiversError {
    fn from(response: Response) -> Self {
        if response.status() == StatusCode::BAD_REQUEST {
            HelldiversError::InvalidWarId(format!("Invalid war ID: {}", response.url()))
        } else {
            HelldiversError::APIError(format!("API error: {}", response.status()))
        }
    }
}