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

const BASE_URL: &str = "https://api.frankfurter.dev";

impl FrankfurterApi {
    pub async fn get_available_currencies() -> Result<Vec<Currency>, String> {
        let response = reqwest::get(format!("{}/v1/currencies", BASE_URL))
            .await
            .map_err(|e| format!("Failed to fetch available currencies: {e}"))?;

        // Deserialize the response into a HashMap JSON response will be in [{symbol: name}, ...]
        let response_text = response
            .text()
            .await
            .map_err(|e| format!("Failed to fetch available currencies: {e}"))?;
        let response_json: HashMap<String, String> = serde_json::from_str(&response_text)
            .map_err(|e| format!("Failed to fetch available currencies: {e}"))?;

        let currencies: Vec<Currency> = response_json
            .iter()
            .map(|(symbol, name)| Currency {
                name: name.clone(),
                symbol: symbol.clone(),
            })
            .collect();
        Ok(currencies)
    }
}
