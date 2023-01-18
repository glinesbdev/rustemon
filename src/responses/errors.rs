use reqwest::StatusCode;
use serde_json::{json, Value};
use std::{env::VarError, fmt::Display};
use url::ParseError;

pub struct RequestError {
    pub message: String,
    pub status: u16,
}

impl<'e> RequestError {
    pub fn new(message: String, status: StatusCode) -> Self {
        let json_err: Value = match serde_json::from_str(&message) {
            Ok(val) => val,
            Err(_) => {
                json!({ "error": { "message": "Unknown error: Could not process request" }})
            }
        };

        Self {
            message: json_err["error"]["message"].to_string(),
            status: status.as_u16(),
        }
    }
}

#[derive(Debug)]
pub struct ResponseError {
    pub message: String,
    pub code: Option<u16>,
}

impl Display for ResponseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Error: {}, Code: {}",
            self.message,
            self.code.unwrap_or_default()
        )
    }
}

impl From<RequestError> for ResponseError {
    fn from(error: RequestError) -> Self {
        Self {
            message: error.message.to_string(),
            code: Some(error.status),
        }
    }
}

impl From<std::io::Error> for ResponseError {
    fn from(error: std::io::Error) -> Self {
        Self {
            message: error.to_string(),
            code: None,
        }
    }
}

impl From<reqwest::Error> for ResponseError {
    fn from(value: reqwest::Error) -> Self {
        Self {
            message: value.to_string(),
            code: None,
        }
    }
}

impl From<VarError> for ResponseError {
    fn from(value: VarError) -> Self {
        Self {
            message: value.to_string(),
            code: None,
        }
    }
}

impl From<std::string::String> for ResponseError {
    fn from(value: std::string::String) -> Self {
        Self {
            message: value,
            code: None,
        }
    }
}

impl From<&str> for ResponseError {
    fn from(value: &str) -> Self {
        Self {
            message: value.to_string(),
            code: None,
        }
    }
}

impl From<ParseError> for ResponseError {
    fn from(value: ParseError) -> Self {
        Self {
            message: value.to_string(),
            code: None,
        }
    }
}
