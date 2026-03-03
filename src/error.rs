use thiserror::Error;

#[derive(Error, Debug)]
pub enum BackpackError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("API Error")]
    Api { status: u16, message: String },
    #[error("Parsing error")]
    Parse(String),

    #[error("Invalid API key")]
    InvalidApiKey(String),

    #[error("Missing API key")]
    MissingApiKey,
}

pub type Result<T> = std::result::Result<T, BackpackError>;
