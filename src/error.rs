use thiserror::Error;

#[derive(Error, Debug)]
pub enum ColorMixerError {
    #[error("Unsupported color: {0}")]
    UnsupportedColor(String),
    
    #[error("Maximum number of colors reached")]
    MaxColorsReached,
    
    #[error("No colors to mix")]
    NoColors,
}

pub type Result<T> = std::result::Result<T, ColorMixerError>;

