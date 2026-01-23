pub mod coingecko;
pub mod error;
pub mod models;
pub mod service;
pub mod traits;

pub use coingecko::CoinGeckoService;
pub use error::CryptoExchangeError;
pub use models::{ExchangeRateRequest, ExchangeRateResponse, SupportedCryptoCoin};
pub use service::CryptoExchangeManager;
pub use traits::CryptoExchange;
