//! JSON-RPC error types

#[cfg(feature = "websocket-client")]
use async_tungstenite::tungstenite::Error as WSError;

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::{self, Display};
use thiserror::Error;

// TODO(thane): Differentiate between RPC response errors and internal crate
//              errors (e.g. domain type-related errors).
/// Tendermint RPC errors
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Error {
    /// Error code
    code: Code,

    /// Error message
    message: String,

    /// Additional data about the error
    data: Option<String>,
}
impl std::error::Error for Error {}

impl Error {
    /// Create a new RPC error
    pub fn new(code: Code, data: Option<String>) -> Error {
        let message = code.to_string();

        Error {
            code,
            message,
            data,
        }
    }

    /// Create a low-level HTTP error
    pub fn http_error(message: impl Into<String>) -> Error {
        Error {
            code: Code::HttpError,
            message: message.into(),
            data: None,
        }
    }

    /// Create a new invalid parameter error
    pub fn invalid_params(data: &str) -> Error {
        Error::new(Code::InvalidParams, Some(data.to_string()))
    }

    /// Create a new websocket error
    pub fn websocket_error(cause: impl Into<String>) -> Error {
        Error::new(Code::WebSocketError, Some(cause.into()))
    }

    /// Create a new method-not-found error
    pub fn method_not_found(name: &str) -> Error {
        Error::new(Code::MethodNotFound, Some(name.to_string()))
    }

    /// Create a new parse error
    pub fn parse_error<E>(error: E) -> Error
    where
        E: Display,
    {
        Error::new(Code::ParseError, Some(error.to_string()))
    }

    /// Create a new server error
    pub fn server_error<D>(data: D) -> Error
    where
        D: Display,
    {
        Error::new(Code::ServerError, Some(data.to_string()))
    }

    /// An internal error occurred within the client.
    pub fn client_internal_error(cause: impl Into<String>) -> Error {
        Error::new(Code::ClientInternalError, Some(cause.into()))
    }

    /// Obtain the `rpc::error::Code` for this error
    pub fn code(&self) -> Code {
        self.code
    }

    /// Borrow the error message (if available)
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Optional additional error message (if available)
    pub fn data(&self) -> Option<&str> {
        self.data.as_ref().map(AsRef::as_ref)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.data {
            Some(data) => write!(
                f,
                "{}: {} (code: {})",
                self.message,
                data,
                self.code.value()
            ),
            None => write!(f, "{} (code: {})", self.message, self.code.value()),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::client_internal_error(e.to_string())
    }
}

impl From<url::ParseError> for Error {
    fn from(e: url::ParseError) -> Self {
        Error::invalid_params(&e.to_string())
    }
}

#[cfg(feature = "http-client")]
impl From<http::Error> for Error {
    fn from(http_error: http::Error) -> Error {
        Error::http_error(http_error.to_string())
    }
}

#[cfg(feature = "http-client")]
impl From<hyper::Error> for Error {
    fn from(hyper_error: hyper::Error) -> Error {
        Error::http_error(hyper_error.to_string())
    }
}

#[cfg(feature = "http-client")]
impl From<http::uri::InvalidUri> for Error {
    fn from(e: http::uri::InvalidUri) -> Self {
        Error::http_error(e.to_string())
    }
}

#[cfg(feature = "websocket-client")]
impl From<WSError> for Error {
    fn from(websocket_error: WSError) -> Error {
        Error::websocket_error(websocket_error.to_string())
    }
}

#[cfg(feature = "http-client-web")]
impl From<wasm_bindgen::JsValue> for Error {
    fn from(e: wasm_bindgen::JsValue) -> Self {
        let error: Result<String, _> = e.into_serde();
        match error {
            Ok(error) =>
                Error::http_error(error),
            Err(serde_error) => {
                Error::http_error(serde_error.to_string())
            }
        }
    }
}

#[cfg(feature = "cli")]
impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::client_internal_error(e.to_string())
    }
}

#[cfg(feature = "cli")]
impl From<tendermint::Error> for Error {
    fn from(e: tendermint::Error) -> Self {
        Error::client_internal_error(e.to_string())
    }
}

/// Tendermint RPC error codes.
///
/// See `func RPC*Error()` definitions in:
/// <https://github.com/tendermint/tendermint/blob/master/rpc/jsonrpc/types/types.go>
#[derive(Copy, Clone, Debug, Eq, Error, Hash, PartialEq, PartialOrd, Ord)]
pub enum Code {
    /// Low-level HTTP error
    #[error("HTTP error")]
    HttpError,

    /// Low-level WebSocket error
    #[error("WebSocket Error")]
    WebSocketError,

    /// An internal error occurred within the client.
    ///
    /// This is an error unique to this client, and is not available in the
    /// [Go client].
    ///
    /// [Go client]: https://github.com/tendermint/tendermint/tree/master/rpc/jsonrpc/client
    #[error("Client internal error")]
    ClientInternalError,

    /// Parse error i.e. invalid JSON (-32700)
    #[error("Parse error. Invalid JSON")]
    ParseError,

    /// Invalid request (-32600)
    #[error("Invalid Request")]
    InvalidRequest,

    /// Method not found error (-32601)
    #[error("Method not found")]
    MethodNotFound,

    /// Invalid parameters (-32602)
    #[error("Invalid params")]
    InvalidParams,

    /// Internal RPC server error (-32603)
    #[error("Internal error")]
    InternalError,

    /// Server error (-32000)
    #[error("Server error")]
    ServerError,

    /// Other error types
    #[error("Error (code: {})", 0)]
    Other(i32),
}

impl Code {
    /// Get the integer error value for this code
    pub fn value(self) -> i32 {
        i32::from(self)
    }
}

impl From<i32> for Code {
    fn from(value: i32) -> Code {
        match value {
            0 => Code::HttpError,
            1 => Code::WebSocketError,
            2 => Code::ClientInternalError,
            -32700 => Code::ParseError,
            -32600 => Code::InvalidRequest,
            -32601 => Code::MethodNotFound,
            -32602 => Code::InvalidParams,
            -32603 => Code::InternalError,
            -32000 => Code::ServerError,
            other => Code::Other(other),
        }
    }
}

impl From<Code> for i32 {
    fn from(code: Code) -> i32 {
        match code {
            Code::HttpError => 0,
            Code::WebSocketError => 1,
            Code::ClientInternalError => 2,
            Code::ParseError => -32700,
            Code::InvalidRequest => -32600,
            Code::MethodNotFound => -32601,
            Code::InvalidParams => -32602,
            Code::InternalError => -32603,
            Code::ServerError => -32000,
            Code::Other(other) => other,
        }
    }
}

impl<'de> Deserialize<'de> for Code {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(Code::from(i32::deserialize(deserializer)?))
    }
}

impl Serialize for Code {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.value().serialize(serializer)
    }
}

#[cfg(test)]
mod tests {
    use super::Code;
    use super::Error;

    #[test]
    fn test_serialize() {
        let expect =
            "{\"code\":-32700,\"message\":\"Parse error. Invalid JSON\",\"data\":\"hello world\"}";
        let pe = Error::parse_error("hello world");
        let pe_json = serde_json::to_string(&pe).expect("could not write JSON");
        assert_eq!(pe_json, expect);
        let res: Error = serde_json::from_str(expect).expect("could not read JSON");
        assert_eq!(res.code, Code::ParseError);
        assert_eq!(res.code.value(), -32700);
        assert_eq!(res.data, Some("hello world".to_string()));
    }
}
