use thiserror::Error;
use crate::events::RustPlusEvent;

/// Result type alias for RustPlus operations
pub type Result<T> = std::result::Result<T, RustPlusError>;

/// Errors that can occur in RustPlus operations
#[derive(Error, Debug)]
pub enum RustPlusError {
    #[error("WebSocket error: {0}")]
    WebSocket(#[from] tokio_tungstenite::tungstenite::Error),
    
    #[error("Protobuf encoding error: {0}")]
    ProtobufEncode(#[from] prost::EncodeError),
    
    #[error("Protobuf decoding error: {0}")]
    ProtobufDecode(#[from] prost::DecodeError),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("JSON serialization error: {0}")]
    Json(#[from] serde_json::Error),
    
    #[error("URL parsing error: {0}")]
    Url(#[from] url::ParseError),
    
    #[error("Connection not established")]
    NotConnected,
    
    #[error("Connection closed")]
    ConnectionClosed,
    
    #[error("Connection error: {0}")]
    ConnectionError(String),
    
    #[error("Connection timeout")]
    Timeout,
    
    #[error("Server error: {0}")]
    ServerError(String),
    
    #[error("Invalid response sequence")]
    InvalidSequence,
    
    #[error("Camera not subscribed")]
    CameraNotSubscribed,
    
    #[error("Invalid camera identifier")]
    InvalidCameraId,
    
    #[error("Image processing error: {0}")]
    ImageProcessing(String),
    
    #[error("Generic error: {0}")]
    Generic(String),
    
    #[error("Event broadcast error: {0}")]
    EventBroadcast(String),
}

impl From<tokio::sync::broadcast::error::SendError<RustPlusEvent>> for RustPlusError {
    fn from(err: tokio::sync::broadcast::error::SendError<RustPlusEvent>) -> Self {
        RustPlusError::EventBroadcast(err.to_string())
    }
}
