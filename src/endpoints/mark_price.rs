use crate::{client::BackpackClient, error::Result, types::mark_price::MarkPrice};

impl BackpackClient {
    pub async fn get_mark_prices(
        &self,
        symbol: Option<&str>,
        market_type: Option<&str>,
    ) -> Result<Vec<MarkPrice>> {
        let mut params = String::new();

        if let Some(s) = symbol {
            params.push_str(&format!("symbol={}", s));
        }
        if let Some(m) = market_type {
            if !params.is_empty() {
                params.push_str("&");
            }
            params.push_str(&format!("marketType={}", m));
        }

        if params.is_empty() {
            let res = self.get::<Vec<MarkPrice>>("/api/v1/markPrices").await?;
            Ok(res)
        } else {
            let res = self.get_with_params("/api/v1/markPrices", &params).await?;
            Ok(res)
        }
    }
}
