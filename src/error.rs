//! JSON-RPC 2.0 errors

extern crate serde;
extern crate serde_json;

use std::error;
use std::result;
use std::fmt;
use std::string::ToString;

use serde_json::{Value, Map, to_value};
use serde_json::value::ToJson;

/// Error Code
#[derive(Clone, PartialEq, Debug)]
pub enum ErrorCode {
    /// Invalid JSON was received by the server.
    /// An error occurred on the server while parsing the JSON text.
    ParseError,
    /// The JSON sent is not a valid Request object.
    InvalidRequest,
    /// The method does not exist / is not available.
    MethodNotFound,
    /// Invalid method parameter(s).
    InvalidParams,
    /// Internal JSON-RPC error.
    InternalError,
    /// Reserved for implementation-defined server-errors.
    ServerError(i64),
}

impl ErrorCode {
    pub fn code(&self) -> i64 {
        match *self {
            ErrorCode::ParseError => -32700,
            ErrorCode::InvalidRequest => -32600,
            ErrorCode::MethodNotFound => -32601,
            ErrorCode::InvalidParams => -32602,
            ErrorCode::InternalError => -32603,
            ErrorCode::ServerError(code) => code,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match *self {
            ErrorCode::ParseError => "Parse error",
            ErrorCode::InvalidRequest => "Invalid request",
            ErrorCode::MethodNotFound => "Method not found",
            ErrorCode::InvalidParams => "Invalid params",
            ErrorCode::InternalError => "Internal error",
            ErrorCode::ServerError(_) => "Server error",
        }
    }
}

impl ToString for ErrorCode {
    fn to_string(&self) -> String {
        String::from(self.as_str())
    }
}

/// Error Object
#[derive(Clone, PartialEq, Debug)]
pub struct Error {
    pub code: i64,
    pub message: String,
    pub data: Option<Value>,
}

impl Error {
    pub fn new(code: ErrorCode) -> Self {
        Error {
            code: code.code(),
            message: code.to_string(),
            data: None,
        }
    }

    pub fn parse_error() -> Self {
        Self::new(ErrorCode::ParseError)
    }

    pub fn invalid_request() -> Self {
        Self::new(ErrorCode::InvalidRequest)
    }

    pub fn method_not_found() -> Self {
        Self::new(ErrorCode::MethodNotFound)
    }

    pub fn invalid_params() -> Self {
        Self::new(ErrorCode::InvalidParams)
    }

    pub fn internal_error() -> Self {
        Self::new(ErrorCode::InternalError)
    }

    pub fn from_value(val: &Value) -> Result<Error> {
        let map = val.as_object();
        if map.is_none() {
            return Err(Error::invalid_request());
        }
        let map: &Map<String, Value> = map.unwrap();
        let code = map.get("code").and_then(|val| {
            if val.is_i64() {
                Some(val.as_i64().unwrap())
            } else {
                None
            }
        });
        if code.is_none() {
            return Err(Error::invalid_request());
        }
        let message = map.get("message").and_then(|val| {
            if val.is_string() {
                Some(val.as_str().unwrap().to_string())
            } else {
                None
            }
        });
        if message.is_none() {
            return Err(Error::invalid_request());
        }
        let data = map.get("data");
        Ok(Error {
            code: code.unwrap(),
            message: message.unwrap(),
            data: if data.is_none() {
                None
            } else {
                Some(data.unwrap().clone())
            },
        })
    }
}

impl ToJson for Error {
    fn to_json(&self) -> Value {
        let mut map = Map::new();
        map.insert("code".to_string(), Value::I64(self.code));
        map.insert("message".to_string(), to_value(&self.message));
        if let Some(ref data) = self.data {
            map.insert("data".to_string(), to_value(data));
        };
        Value::Object(map)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.to_json(), f)
    }
}

pub type Result<T> = result::Result<T, Error>;
