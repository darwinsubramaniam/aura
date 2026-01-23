use crate::db::RowId;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Deserialize, Serialize, sqlx::Type, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
#[sqlx(rename_all = "lowercase", type_name = "TEXT")]
pub enum CryptoAssetType {
    #[serde(rename = "stablecoin")]
    #[sqlx(rename = "stablecoin")]
    Stable,
    #[serde(rename = "cryptocoin")]
    #[sqlx(rename = "cryptocoin")]
    Crypto,
    #[serde(rename = "nft")]
    #[sqlx(rename = "nft")]
    Nft,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct CryptoAsset {
    pub id: RowId,
    pub name: String,
    pub symbol: String,
    pub kind: CryptoAssetType,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCryptoAsset {
    pub name: String,
    pub symbol: String,
    pub kind: CryptoAssetType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateCryptoAsset {
    pub name: Option<String>,
    pub symbol: Option<String>,
    pub kind: Option<CryptoAssetType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CryptoAssetPagination {
    pub total_count: i64,
    pub assets: Vec<CryptoAsset>,
}
