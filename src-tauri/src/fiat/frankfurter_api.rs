use reqwest;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Frankfurter API - https://frankfurter.dev/
pub struct FrankfurterApi;

#[derive(Debug, Serialize, Deserialize)]
pub struct Currency {
    pub name: String,
    pub symbol: String,
}

const BASE_URL: &str = "https://api.frankfurter.dev";

impl FrankfurterApi {
    pub async fn get_available_currencies() -> Result<Vec<Currency>, reqwest::Error> {
        let response = reqwest::get(format!("{}/v1/currencies", BASE_URL)).await?;

        // Deserialize the response into a HashMap JSON response will be in [{symbol: name}, ...]
        let response: HashMap<String, String> = response.json().await?;

        let currencies: Vec<Currency> = response
            .iter()
            .map(|(symbol, name)| Currency {
                name: name.clone(),
                symbol: symbol.clone(),
            })
            .collect();
        Ok(currencies)
    }
}
