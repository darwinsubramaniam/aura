use crate::crypto_exchange::{
    CryptoExchange, CryptoExchangeError, ExchangeRateRequest, ExchangeRateResponse, SupportedCoin,
};
use async_trait::async_trait;
use reqwest::{Client, StatusCode};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::time::Duration;

const COINGECKO_API_URL: &str = "https://api.coingecko.com/api/v3";

#[derive(Clone)]
pub struct CoinGeckoService {
    client: Client,
}

impl CoinGeckoService {
    pub fn new() -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(10))
            .user_agent("AuraApp/0.1.0") // Added User-Agent
            .build()
            .unwrap_or_else(|_| Client::new());
        Self { client }
    }

    async fn handle_error(&self, response: reqwest::Response) -> CryptoExchangeError {
        match response.status() {
            StatusCode::TOO_MANY_REQUESTS => CryptoExchangeError::RateLimitExceeded,
            StatusCode::NOT_FOUND => {
                CryptoExchangeError::AssetNotFound("Resource not found".to_string())
            }
            status => {
                let text = response.text().await.unwrap_or_default();
                CryptoExchangeError::ApiError(format!("Status: {}, Body: {}", status, text))
            }
        }
    }
}

#[derive(Deserialize)]
struct CoinListEntry {
    id: String,
    symbol: String,
    name: String,
}

#[async_trait]
impl CryptoExchange for CoinGeckoService {
    fn id(&self) -> &'static str {
        "coingecko"
    }

    async fn list_supported_coins(&self) -> Result<Vec<SupportedCoin>, CryptoExchangeError> {
        let url = format!("{}/coins/list", COINGECKO_API_URL);
        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| CryptoExchangeError::Network(e.to_string()))?;

        if !response.status().is_success() {
            return Err(self.handle_error(response).await);
        }

        let coins: Vec<CoinListEntry> = response.json().await.map_err(|e| {
            CryptoExchangeError::ApiError(format!("Failed to parse coin list: {}", e))
        })?;

        Ok(coins
            .into_iter()
            .map(|c| SupportedCoin {
                id: c.id,
                symbol: c.symbol,
                name: c.name,
            })
            .collect())
    }

    async fn get_exchange_rate(
        &self,
        request: ExchangeRateRequest,
    ) -> Result<ExchangeRateResponse, CryptoExchangeError> {
        if let Some(date) = request.date {
            // Historical Price
            let date_str = date.format("%d-%m-%Y").to_string();
            let url = format!(
                "{}/coins/{}/history?date={}&localization=false",
                COINGECKO_API_URL, request.coin_id, date_str
            );

            let response = self
                .client
                .get(&url)
                .send()
                .await
                .map_err(|e| CryptoExchangeError::Network(e.to_string()))?;

            if !response.status().is_success() {
                return Err(self.handle_error(response).await);
            }

            let json: Value = response.json().await.map_err(|e| {
                CryptoExchangeError::ApiError(format!("Failed to parse history response: {}", e))
            })?;

            if let Some(err) = json.get("error") {
                return Err(CryptoExchangeError::ApiError(err.to_string()));
            }

            let rate = json
                .get("market_data")
                .and_then(|md| md.get("current_price"))
                .and_then(|cp| cp.get(request.fiat_currency.to_lowercase().as_str()))
                .and_then(|v| {
                    if v.is_f64() {
                        use rust_decimal::prelude::FromPrimitive;
                        Decimal::from_f64(v.as_f64().unwrap())
                    } else if v.is_i64() {
                        Some(Decimal::from(v.as_i64().unwrap()))
                    } else {
                        None
                    }
                });

            match rate {
                Some(r) => Ok(ExchangeRateResponse { rate: r, date }),
                None => Err(CryptoExchangeError::AssetNotFound(format!(
                    "Price not found for {} on {}",
                    request.coin_id, date_str
                ))),
            }
        } else {
            // Current Price
            let fiat_lower = request.fiat_currency.to_lowercase();
            let url = format!(
                "{}/simple/price?ids={}&vs_currencies={}",
                COINGECKO_API_URL, request.coin_id, fiat_lower
            );

            let response = self
                .client
                .get(&url)
                .send()
                .await
                .map_err(|e| CryptoExchangeError::Network(e.to_string()))?;

            if !response.status().is_success() {
                return Err(self.handle_error(response).await);
            }

            let json: Value = response.json().await.map_err(|e| {
                CryptoExchangeError::ApiError(format!("Failed to parse price response: {}", e))
            })?;

            let rate = json
                .get(&request.coin_id)
                .and_then(|c| c.get(&fiat_lower))
                .and_then(|v| {
                    if v.is_f64() {
                        use rust_decimal::prelude::FromPrimitive;
                        Decimal::from_f64(v.as_f64().unwrap())
                    } else if v.is_i64() {
                        Some(Decimal::from(v.as_i64().unwrap()))
                    } else {
                        None
                    }
                });

            match rate {
                Some(r) => Ok(ExchangeRateResponse {
                    rate: r,
                    date: chrono::Local::now().date_naive(),
                }),
                None => Err(CryptoExchangeError::AssetNotFound(format!(
                    "Price not found for {}",
                    request.coin_id
                ))),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[tokio::test]
    #[ignore] // Integration test: hits external API
    async fn test_get_exchange_rate_current() {
        let service = CoinGeckoService::new();
        let request = ExchangeRateRequest {
            coin_id: "bitcoin".to_string(),
            fiat_currency: "usd".to_string(),
            date: None,
        };

        let result = service.get_exchange_rate(request).await;
        assert!(
            result.is_ok(),
            "Failed to get current rate: {:?}",
            result.err()
        );
        let response = result.unwrap();
        assert!(response.rate > Decimal::new(0, 0));
        println!("Bitcoin Price: {}", response.rate);
    }

    #[tokio::test]
    #[ignore] // Integration test: hits external API
    async fn test_get_exchange_rate_history() {
        let service = CoinGeckoService::new();
        // Public API limits history to 365 days. Use a date 30 days ago.
        let date = chrono::Local::now().date_naive() - chrono::Duration::days(30);
        let request = ExchangeRateRequest {
            coin_id: "bitcoin".to_string(),
            fiat_currency: "usd".to_string(),
            date: Some(date),
        };

        let result = service.get_exchange_rate(request).await;
        assert!(
            result.is_ok(),
            "Failed to get history rate: {:?}",
            result.err()
        );
        let response = result.unwrap();
        assert!(response.rate > Decimal::new(10000, 0));
        assert_eq!(response.date, date);
        println!("Bitcoin Price on {}: {}", date, response.rate);
    }

    #[tokio::test]
    #[ignore] // Integration test: hits external API
    async fn test_list_supported_coins() {
        let service = CoinGeckoService::new();
        let result = service.list_supported_coins().await;
        assert!(result.is_ok(), "Failed to list coins: {:?}", result.err());
        let coins = result.unwrap();
        assert!(!coins.is_empty());
        assert!(coins.iter().any(|c| c.id == "bitcoin"));
        println!("Found {} coins", coins.len());
    }

    #[tokio::test]
    #[ignore] // Integration test: hits external API
    async fn test_error_handling_invalid_coin() {
        let service = CoinGeckoService::new();
        let request = ExchangeRateRequest {
            coin_id: "invalid-coin-id-12345".to_string(),
            fiat_currency: "usd".to_string(),
            date: None,
        };

        let result = service.get_exchange_rate(request).await;
        // CoinGecko usually returns empty or success but no value for invalid ID in simple price,
        // OR it returns an empty object which our code parses as AssetNotFound.
        // Let's verify our code handles it gracefully.
        assert!(result.is_err());
    }
}
