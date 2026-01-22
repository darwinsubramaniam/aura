use crate::{
    crypto_asset::models::{CryptoAsset, UpdateCryptoAsset},
    db::{Db, RowId},
    utils::pagination_model::Pagination,
};

pub struct CryptoAssetService {}

impl CryptoAssetService {
    pub async fn get(db: &Db, pagination: &Pagination) {
        todo!()
    }

    pub async fn set(db: &Db, data: &CryptoAsset) {
        todo!()
    }

    pub async fn update(db: &Db, id: RowId, data: &UpdateCryptoAsset) {
        todo!()
    }

    pub async fn delete(db: &Db, id: RowId) {
        todo!()
    }
}
