use super::error::CryptoExchangeError;
use super::models::{ExchangeRateRequest, ExchangeRateResponse, SupportedCoin};
use async_trait::async_trait;

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait CryptoExchange: Send + Sync {
    async fn list_supported_coins(&self) -> Result<Vec<SupportedCoin>, CryptoExchangeError>;
    async fn get_exchange_rate(
        &self,
        request: ExchangeRateRequest,
    ) -> Result<ExchangeRateResponse, CryptoExchangeError>;
    fn id(&self) -> &'static str;
}
