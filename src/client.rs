use crate::error::{BackpackError, Result};

use reqwest::Client;
const BASE_URL: &str = "https://api.backpack.exchange";

pub struct BackpackClient {
    http: Client,
    base_url: String,
}

impl BackpackClient {
    pub fn new() -> Self {
        Self {
            http: Client::new(),
            base_url: BASE_URL.to_string(),
        }
    }

    pub(crate) async fn get<T>(&self, path: &str) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let url = format!("{}{}", self.base_url, path);

        let response = self.http.get(&url).send().await?;

        let status = response.status();

        if !status.is_success() {
            let message = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());

            return Err(BackpackError::Api {
                status: status.as_u16(),
                message,
            });
        }

        let text = response.text().await.map_err(|e| BackpackError::Parse(e.to_string()))?;

        let data = serde_json::from_str::<T>(&text)
            .map_err(|e| BackpackError::Parse(e.to_string()))?;

        Ok(data)
    }
}

impl Default for BackpackClient {
    fn default() -> Self {
        Self::new()
    }
}
