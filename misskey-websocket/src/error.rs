use std::sync::Arc;

use async_tungstenite::tungstenite;
use derive_more::Display;

/// Possible errors from WebSocket client.
#[derive(Debug, Display, Clone)]
pub enum Error {
    /// Errors from underlying [tungstenite](https://docs.rs/tungstenite) library.
    #[display(fmt = "websocket error: {}", _0)]
    WebSocket(Arc<tungstenite::Error>),
    /// Received unexpected message from server.
    #[display(fmt = "websocket unexpected message: {}", _0)]
    UnexpectedMessage(tungstenite::Message),
    /// JSON encode/decode error.
    #[display(fmt = "JSON error: {}", _0)]
    Json(Arc<serde_json::Error>),
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::WebSocket(e) => Some(e.as_ref()),
            Error::UnexpectedMessage(_) => None,
            Error::Json(e) => Some(e.as_ref()),
        }
    }
}

impl From<tungstenite::Error> for Error {
    fn from(err: tungstenite::Error) -> Error {
        Error::WebSocket(Arc::new(err))
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Error {
        Error::Json(Arc::new(err))
    }
}

/// Specialized Result type for operations on [`WebSocketClient`][`crate::WebSocketClient`].
pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests {
    use super::Error;

    #[test]
    fn test_send() {
        fn assert_send<T: Send>() {}
        assert_send::<Error>();
    }

    #[test]
    fn test_sync() {
        fn assert_send<T: Sync>() {}
        assert_send::<Error>();
    }
}
