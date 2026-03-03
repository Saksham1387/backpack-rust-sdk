use crate::{client::BackpackClient, error::Result, types::ticker::Ticker};

impl BackpackClient {
    pub async fn get_ticker(&self, symbol: &str, interval: Option<&str>) -> Result<Ticker> {
        let params = match interval {
            Some(i) => format!("symbol={}&interval={}", symbol, i),
            None => format!("symbol={}", symbol),
        };
        let res = self.get_with_params("/api/v1/ticker", &params).await?;
        Ok(res)
    }

    pub async fn get_tickers(&self, interval: Option<&str>) -> Result<Vec<Ticker>> {
        let params = match interval {
            Some(i) => format!("interval={}", i),
            None => String::new(),
        };

        if params.is_empty() {
            let res = self.get::<Vec<Ticker>>("/api/v1/tickers").await?;
            Ok(res)
        } else {
            let res = self.get_with_params("/api/v1/tickers", &params).await?;
            Ok(res)
        }
    }
}
