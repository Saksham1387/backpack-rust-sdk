use crate::{client::BackpackClient, error::Result, types::klines::Kline};

impl BackpackClient {
    pub async fn get_klines(
        &self,
        symbol: &str,
        interval: &str,
        start_time: u64,
        end_time: Option<u64>,
        price_type: Option<&str>,
    ) -> Result<Vec<Kline>> {
        let mut params = format!("symbol={}&interval={}&startTime={}", symbol, interval, start_time);
        if let Some(e) = end_time {
            params.push_str(&format!("&endTime={}", e));
        }
        if let Some(p) = price_type {
            params.push_str(&format!("&priceType={}", p));
        }
        let res = self.get_with_params("/api/v1/klines", &params).await?;
        Ok(res)
    }
}
