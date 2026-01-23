use crate::crypto_asset::error::CryptoAssetError;
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error, Serialize, PartialEq)]
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

impl From<CryptoAssetError> for CryptoExchangeError {
    fn from(err: CryptoAssetError) -> Self {
        match err {
            CryptoAssetError::AssetNotFound(msg) => CryptoExchangeError::AssetNotFound(msg),
            CryptoAssetError::Other(msg) => CryptoExchangeError::Other(msg),
        }
    }
}

impl From<reqwest::Error> for CryptoExchangeError {
    fn from(err: reqwest::Error) -> Self {
        CryptoExchangeError::Network(err.to_string())
    }
}

impl From<serde_json::Error> for CryptoExchangeError {
    fn from(err: serde_json::Error) -> Self {
        CryptoExchangeError::Other(err.to_string())
    }
}

impl From<sqlx::Error> for CryptoExchangeError {
    fn from(err: sqlx::Error) -> Self {
        CryptoExchangeError::Other(err.to_string())
    }
}

impl From<String> for CryptoExchangeError {
    fn from(err: String) -> Self {
        CryptoExchangeError::Other(err)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_string() {
        let err = String::from("custom error");
        let crypto_exchange_err = CryptoExchangeError::from(err);
        assert_eq!(
            crypto_exchange_err,
            CryptoExchangeError::Other("custom error".to_string())
        );
    }
}
