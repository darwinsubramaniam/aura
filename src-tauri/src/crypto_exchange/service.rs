use super::error::CryptoExchangeError;
use super::models::{ExchangeRateRequest, ExchangeRateResponse, SupportedCoin};
use super::traits::CryptoExchange;

pub struct CryptoExchangeManager {
    providers: Vec<Box<dyn CryptoExchange>>,
}

impl CryptoExchangeManager {
    pub fn new(providers: Vec<Box<dyn CryptoExchange>>) -> Self {
        Self { providers }
    }

    pub async fn get_exchange_rate(
        &self,
        request: ExchangeRateRequest,
    ) -> Result<ExchangeRateResponse, CryptoExchangeError> {
        let mut last_error = CryptoExchangeError::Other("No providers configured".to_string());

        for provider in &self.providers {
            match provider.get_exchange_rate(request.clone()).await {
                Ok(rate) => return Ok(rate),
                Err(e) => {
                    eprintln!(
                        "Provider {} failed to get rate for {}: {}",
                        provider.id(),
                        request.coin_id,
                        e
                    );
                    last_error = e;
                    // Continue to next provider
                }
            }
        }
        Err(last_error)
    }

    pub async fn list_supported_coins(&self) -> Result<Vec<SupportedCoin>, CryptoExchangeError> {
        // TODO: Implement of Asset and Asset-Exchange-Mapping is pending for supported coins
        // set this to todo!() for now as DB on for Asset and exchange-Mapping implementation is pending
        todo!("Implement database storage for supported coins")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto_exchange::traits::MockCryptoExchange;
    use chrono::NaiveDate;
    use rust_decimal::Decimal;

    #[tokio::test]
    async fn test_get_exchange_rate_success_first_provider() {
        let mut mock_provider = MockCryptoExchange::new();
        mock_provider.expect_id().return_const("mock_provider_1");
        mock_provider
            .expect_get_exchange_rate()
            .times(1)
            .returning(|_| {
                Ok(ExchangeRateResponse {
                    rate: Decimal::new(50000, 0),
                    date: NaiveDate::from_ymd_opt(2023, 1, 1).unwrap(),
                })
            });

        let manager = CryptoExchangeManager::new(vec![Box::new(mock_provider)]);

        let request = ExchangeRateRequest {
            coin_id: "bitcoin".to_string(),
            fiat_currency: "usd".to_string(),
            date: None,
        };

        let result = manager.get_exchange_rate(request).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().rate, Decimal::new(50000, 0));
    }

    #[tokio::test]
    async fn test_get_exchange_rate_fallback() {
        let mut mock_provider1 = MockCryptoExchange::new();
        mock_provider1.expect_id().return_const("mock_provider_1");
        mock_provider1
            .expect_get_exchange_rate()
            .times(1)
            .returning(|_| Err(CryptoExchangeError::Network("Timeout".to_string())));

        let mut mock_provider2 = MockCryptoExchange::new();
        mock_provider2.expect_id().return_const("mock_provider_2");
        mock_provider2
            .expect_get_exchange_rate()
            .times(1)
            .returning(|_| {
                Ok(ExchangeRateResponse {
                    rate: Decimal::new(50000, 0),
                    date: NaiveDate::from_ymd_opt(2023, 1, 1).unwrap(),
                })
            });

        let manager =
            CryptoExchangeManager::new(vec![Box::new(mock_provider1), Box::new(mock_provider2)]);

        let request = ExchangeRateRequest {
            coin_id: "bitcoin".to_string(),
            fiat_currency: "usd".to_string(),
            date: None,
        };

        let result = manager.get_exchange_rate(request).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().rate, Decimal::new(50000, 0));
    }

    #[tokio::test]
    async fn test_get_exchange_rate_all_fail() {
        let mut mock_provider1 = MockCryptoExchange::new();
        mock_provider1.expect_id().return_const("mock_provider_1");
        mock_provider1
            .expect_get_exchange_rate()
            .times(1)
            .returning(|_| Err(CryptoExchangeError::Network("Timeout".to_string())));

        let manager = CryptoExchangeManager::new(vec![Box::new(mock_provider1)]);

        let request = ExchangeRateRequest {
            coin_id: "bitcoin".to_string(),
            fiat_currency: "usd".to_string(),
            date: None,
        };

        let result = manager.get_exchange_rate(request).await;
        assert!(result.is_err());
        match result.unwrap_err() {
            CryptoExchangeError::Network(_) => (),
            _ => panic!("Expected Network error"),
        }
    }
}
