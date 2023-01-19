use reqwest::StatusCode;
use serde_json::{json, Value};

pub struct RequestError {
    pub message: String,
    pub status: u16,
}

impl RequestError {
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
    pub code: u16,
}

impl std::fmt::Display for ResponseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Error: {}, Code: {}",
            self.message,
            self.code
        )
    }
}

impl From<RequestError> for ResponseError {
    fn from(error: RequestError) -> Self {
        Self {
            message: error.message.to_string(),
            code: error.status,
        }
    }
}

impl From<reqwest::Error> for ResponseError {
    fn from(value: reqwest::Error) -> Self {
        Self {
            message: value.to_string(),
            code: value.status().unwrap_or_default().as_u16(),
        }
    }
}
