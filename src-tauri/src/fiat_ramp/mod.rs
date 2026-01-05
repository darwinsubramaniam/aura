pub mod command;
use crate::db::Db;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use std::str::FromStr;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RampKind {
    Deposit,
    Withdraw,
}

impl ToString for RampKind {
    fn to_string(&self) -> String {
        match self {
            RampKind::Deposit => "deposit".to_string(),
            RampKind::Withdraw => "withdraw".to_string(),
        }
    }
}

impl FromStr for RampKind {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "deposit" => Ok(RampKind::Deposit),
            "withdraw" => Ok(RampKind::Withdraw),
            _ => Err(format!("invalid ramp kind: {s}")),
        }
    }
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct FiatRamp {
    pub id: Uuid,
    pub fiat_id: i64,
    pub fiat_amount: f64,
    pub date: chrono::NaiveDate,
    pub kind: String,
    pub via_exchange: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateFiatRamp {
    pub fiat_id: i64,
    pub fiat_amount: f64,
    pub date: chrono::NaiveDate,
    pub via_exchange: String,
    pub kind: RampKind,
}

pub struct FiatRampService {}

impl FiatRampService {
    pub async fn create_fiat_ramp(create_fiat_ramp: CreateFiatRamp, db: &Db) -> Result<(), String> {
        let id = Uuid::now_v7().to_string();
        println!("create_fiat_ramp: {create_fiat_ramp:?}");
        let mut tx =
            db.0.begin()
                .await
                .map_err(|e| format!("failed to begin transaction: {e}"))?;

        sqlx::query("INSERT INTO fiat_ramp (id, fiat_id, fiat_amount, date, via_exchange, kind) VALUES (?, ?, ?, ?, ?, ?)")
        .bind(id)
        .bind(create_fiat_ramp.fiat_id)
        .bind(create_fiat_ramp.fiat_amount)
        .bind(create_fiat_ramp.date)
        .bind(create_fiat_ramp.via_exchange)
        .bind(create_fiat_ramp.kind.to_string())
        .execute(&mut *tx)
        .await
        .map_err(|e| format!("failed to insert into fiat_ramp table: {e}"))?;

        tx.commit()
            .await
            .map_err(|e| format!("failed to commit transaction: {e}"))?;
        println!("fiat ramp created successfully");
        Ok(())
    }
}
