use thiserror::Error;

#[derive(Error, Debug)]
pub enum ColorMixerError {
    #[error("Invalid color format: {0}")]
    InvalidColorFormat(String),
    
    #[error("Failed to parse hex color: {0}")]
    HexParseError(#[from] hex::FromHexError),
    
    #[error("Color not supported: {0}")]
    UnsupportedColor(String),
    
    #[error("Maximum number of colors reached")]
    MaxColorsReached,
    
    #[error("No colors in mixer")]
    NoColors,
    
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
    
    #[error("Internal error: {0}")]
    InternalError(String),
}

pub type Result<T> = std::result::Result<T, ColorMixerError>;

