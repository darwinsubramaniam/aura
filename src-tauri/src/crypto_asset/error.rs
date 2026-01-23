use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error, Serialize, PartialEq)]
pub enum CryptoAssetError {
    #[error("Asset not found: {0}")]
    AssetNotFound(String),
    #[error("Other error: {0}")]
    Other(String),
}
