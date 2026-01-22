use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error, Serialize)]
pub enum CryptoExchangeError {
    #[error("Network error: {0}")]
    Network(String),
    #[error("Rate limit exceeded")]
    RateLimitExceeded,
    #[error("Asset not found: {0}")]
    AssetNotFound(String),
    #[error("API error: {0}")]
    ApiError(String),
    #[error("Other error: {0}")]
    Other(String),
}
