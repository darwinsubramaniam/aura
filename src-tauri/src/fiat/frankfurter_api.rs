use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tauri_plugin_http::reqwest;

/// Frankfurter API - https://frankfurter.dev/
pub struct FrankfurterApi;

#[derive(Debug, Serialize, Deserialize)]
pub struct Currency {
    pub name: String,
    pub symbol: String,
}

/// Fiat currency symbol
pub type FiatSymbol = String;

#[derive(Debug, Serialize, Deserialize)]
pub struct LatestRates {
    pub base: String,
    pub date: chrono::NaiveDate,
    pub rates: HashMap<FiatSymbol, f64>,
}

const BASE_URL: &str = "https://api.frankfurter.dev";

impl FrankfurterApi {
    pub async fn get_available_currencies() -> Result<Vec<Currency>> {
        let response = reqwest::get(format!("{}/v1/currencies", BASE_URL))
            .await
            .context("Failed to fetch available currencies")?;

        // Deserialize the response into a HashMap JSON response will be in [{symbol: name}, ...]
        let response_text = response
            .text()
            .await
            .context("Failed to fetch available currencies")?;
        let response_json: HashMap<String, String> =
            serde_json::from_str(&response_text).context("Failed to fetch available currencies")?;

        let mut currencies: Vec<Currency> = response_json
            .iter()
            .map(|(symbol, name)| Currency {
                name: name.clone(),
                symbol: symbol.clone(),
            })
            .collect();
        // sort currencies by symbol
        currencies.sort_by(|a, b| a.symbol.cmp(&b.symbol));
        Ok(currencies)
    }

    pub async fn get_latest_rates(
        base: &str,
        date: Option<&chrono::NaiveDate>,
    ) -> Result<LatestRates> {
        let url = if let Some(date) = date {
            format!("{BASE_URL}/v1/{date}?base={base}")
        } else {
            format!("{BASE_URL}/v1/latest?base={base}")
        };
        let response = reqwest::get(url)
            .await
            .context("Failed to fetch latest rates")?;
        // Deserialize the response - using this method because the tauri plugin http reqwest does not support deserializing the to using .json()
        let response_text = response
            .text()
            .await
            .context("Failed to fetch latest rates")?;
        let response_json: LatestRates =
            serde_json::from_str(&response_text).context("Failed to fetch latest rates")?;
        Ok(response_json)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test_get_available_currencies() {
        let currencies = FrankfurterApi::get_available_currencies().await.unwrap();

        // test if there is MYR in the list
        assert!(currencies.iter().any(|currency| currency.symbol == "MYR"));

        // ensure the list is sorted by symbol
        assert!(currencies.windows(2).all(|w| w[0].symbol < w[1].symbol));
    }

    #[tokio::test]
    async fn test_get_latest_rates() {
        let rates = FrankfurterApi::get_latest_rates("MYR", None).await.unwrap();

        assert!(!rates.date.to_string().is_empty());

        assert!(!rates.rates.is_empty());
        assert!(rates.rates.contains_key("SGD"));
    }

    #[tokio::test]
    async fn test_get_latest_rates_with_date() {
        let rates = FrankfurterApi::get_latest_rates(
            "MYR",
            Some(&chrono::NaiveDate::from_ymd_opt(2026, 2, 1).unwrap()),
        )
        .await
        .unwrap();

        assert!(!rates.date.to_string().is_empty());

        // test if there is some rates contain SGD
        assert!(!rates.rates.is_empty());
        assert!(rates.rates.contains_key("SGD"));
    }
}
