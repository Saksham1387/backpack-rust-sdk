use crate::{
    error::{BackpackError, Result},
    signer::Signer,
};

use reqwest::{Client, Response};
use serde::Serialize;
const BASE_URL: &str = "https://api.backpack.exchange";
const BASE_URL_WS: &str = "wss://ws.backpack.exchange";

pub struct BackpackClient {
    http: Client,
    base_url: String,
    ws_url: Option<String>,

    signer: Option<Signer>,
}

impl BackpackClient {
    pub fn new() -> Self {
        Self {
            http: Client::new(),
            base_url: BASE_URL.to_string(),
            signer: None,
            ws_url: Some(BASE_URL_WS.to_string()),
        }
    }

    pub fn with_signer(private_key_b64: &str) -> Result<Self> {
        let signer = Signer::from_base64(private_key_b64)?;

        Ok(Self {
            http: Client::new(),
            base_url: BASE_URL.to_string(),
            signer: Some(signer),
            ws_url: Some(BASE_URL_WS.to_string()),
        })
    }

    pub(crate) async fn signed_get<T>(
        &self,
        instruction: &str,
        path: &str,
        params: &str,
    ) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let signer = self.signer.as_ref().ok_or(BackpackError::MissingApiKey)?;

        let headers = signer.sign(instruction, params);

        let url = if params.is_empty() {
            format!("{}{}", self.base_url, path)
        } else {
            format!("{}{}?{}", self.base_url, path, params)
        };

        let response = self
            .http
            .get(&url)
            .header("X-API-Key", &headers.x_api_key)
            .header("X-Signature", &headers.x_signature)
            .header("X-Timestamp", &headers.x_timestamp)
            .header("X-Window", &headers.x_window)
            .send()
            .await?;

        Self::parse_response(response).await
    }

    pub(crate) async fn get<T>(&self, path: &str) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        self.get_with_params(path, "").await
    }

    pub(crate) async fn get_with_params<T>(&self, path: &str, params: &str) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let url = if params.is_empty() {
            format!("{}{}", self.base_url, path)
        } else {
            format!("{}{}?{}", self.base_url, path, params)
        };

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

        let text = response
            .text()
            .await
            .map_err(|e| BackpackError::Parse(e.to_string()))?;

        let data =
            serde_json::from_str::<T>(&text).map_err(|e| BackpackError::Parse(e.to_string()))?;

        Ok(data)
    }

    async fn parse_response<T>(response: Response) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
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

        let body = response.text().await?;
        println!("Response body: {}", body);

        serde_json::from_str(&body).map_err(|e| BackpackError::Parse(e.to_string()))
    }

    pub(crate) async fn signed_post<T, B>(
        &self,
        instruction: &str,
        path: &str,
        params: &str, // sorted query string (for signing)
        body: &B,     // the actual request body struct (serialized to JSON)
    ) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
        B: Serialize,
    {
        let signer = self.signer.as_ref().ok_or(BackpackError::MissingApiKey)?;
        let headers = signer.sign(instruction, params);
        let url = format!("{}{}", self.base_url, path);

        let response = self
            .http
            .post(&url)
            .header("X-API-Key", &headers.x_api_key)
            .header("X-Signature", &headers.x_signature)
            .header("X-Timestamp", &headers.x_timestamp)
            .header("X-Window", &headers.x_window)
            .json(body) // serializes body to JSON, sets Content-Type: application/json
            .send()
            .await?;

        Self::parse_response(response).await
    }

    pub(crate) async fn signed_delete<T, B>(
        &self,
        instruction: &str,
        path: &str,
        params: &str,
        body: &B,
    ) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
        B: Serialize,
    {
        let signer = self.signer.as_ref().ok_or(BackpackError::MissingApiKey)?;
        let headers = signer.sign(instruction, params);
        let url = format!("{}{}", self.base_url, path);

        let response = self
            .http
            .delete(&url)
            .header("X-API-Key", &headers.x_api_key)
            .header("X-Signature", &headers.x_signature)
            .header("X-Timestamp", &headers.x_timestamp)
            .header("X-Window", &headers.x_window)
            .json(body)
            .send()
            .await?;

        Self::parse_response(response).await
    }
}

impl Default for BackpackClient {
    fn default() -> Self {
        Self::new()
    }
}
