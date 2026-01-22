use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, sqlx::Type)]
#[serde(rename_all = "lowercase")]
#[sqlx(rename_all = "lowercase", type_name = "TEXT")]
pub enum CryptoAssetType {
    Stable,
    Crypto,
    Nft,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CryptoAsset {
    pub name: String,
    pub symbol: String,
    pub kind: CryptoAssetType,
}

pub struct UpdateCryptoAsset {
    pub name: Option<String>,
    pub symbol: Option<String>,
    pub kind: Option<CryptoAssetType>,
}
