use crate::{client::BackpackClient, error::Result, types::markets::Market};

impl BackpackClient {
    pub async fn get_markets(&self, market_type: Option<Vec<String>>) -> Result<Vec<Market>> {
        let params = match market_type {
            Some(types) => {
                let types_str = types.join(",");
                format!("marketType={}", types_str)
            }
            None => String::new(),
        };

        if params.is_empty() {
            let res = self.get::<Vec<Market>>("/api/v1/markets").await?;
            Ok(res)
        } else {
            let res = self.get_with_params("/api/v1/markets", &params).await?;
            Ok(res)
        }
    }

    pub async fn get_market(&self, symbol: &str) -> Result<Market> {
        let params = format!("symbol={}", symbol);
        let res = self.get_with_params("/api/v1/market", &params).await?;
        Ok(res)
    }
}
