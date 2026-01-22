use crate::fiat_exchanger::{Currency, FiatExchanger, Rates};
use anyhow::{Context, Result};
use std::collections::HashMap;
use tauri_plugin_http::reqwest;

/// Frankfurter API - https://frankfurter.dev/
pub struct FrankfurterExchangerApi {
    base_url: String,
}

impl Default for FrankfurterExchangerApi {
    fn default() -> Self {
        Self {
            base_url: "https://api.frankfurter.dev".to_string(),
        }
    }
}
impl FiatExchanger for FrankfurterExchangerApi {
    async fn get_available_currencies(&self) -> Result<Vec<Currency>> {
        let response = reqwest::get(format!("{}/v1/currencies", self.base_url))
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

    async fn get_latest_rates(
        &self,
        base: &str,
        date: Option<&chrono::NaiveDate>,
    ) -> Result<Rates> {
        let url = if let Some(date) = date {
            format!("{}/v1/{date}?base={base}", self.base_url)
        } else {
            format!("{}/v1/latest?base={base}", self.base_url)
        };
        let response = reqwest::get(url)
            .await
            .context("Failed to fetch latest rates")?;
        // Deserialize the response - using this method because the tauri plugin http reqwest does not support deserializing the to using .json()
        let response_text = response
            .text()
            .await
            .context("Failed to fetch latest rates")?;
        let response_json: Rates =
            serde_json::from_str(&response_text).context("Failed to fetch latest rates")?;
        Ok(response_json)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test_get_available_currencies() {
        let api = FrankfurterExchangerApi::default();
        let currencies = api.get_available_currencies().await.unwrap();

        // test if there is MYR in the list
        assert!(currencies.iter().any(|currency| currency.symbol == "MYR"));

        // ensure the list is sorted by symbol
        assert!(currencies.windows(2).all(|w| w[0].symbol < w[1].symbol));
    }

    #[tokio::test]
    async fn test_get_latest_rates() {
        let api = FrankfurterExchangerApi::default();
        let rates = api.get_latest_rates("MYR", None).await.unwrap();

        assert!(!rates.date.to_string().is_empty());

        assert!(!rates.rates.is_empty());
        assert!(rates.rates.contains_key("SGD"));
    }

    #[tokio::test]
    async fn test_get_latest_rates_with_date() {
        let api = FrankfurterExchangerApi::default();
        let rates = api
            .get_latest_rates(
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
