use crate::{BackpackClient, Result, Trade};

impl BackpackClient {
    pub async fn get_recent_trades(&self, symbol: &str, limit: Option<u16>) -> Result<Vec<Trade>> {
        let params = match limit {
            Some(l) => format!("symbol={}&limit={}", symbol, l),
            None => format!("symbol={}", symbol),
        };
        let res = self.get_with_params("/api/v1/trades", &params).await?;

        Ok(res)
    }

    pub async fn get_historical_trades(
        &self,
        symbol: &str,
        limit: Option<u16>,
        offset: Option<u64>,
    ) -> Result<Vec<Trade>> {
        let mut params = format!("symbol={}", symbol);
        if let Some(l) = limit {
            params.push_str(&format!("&limit={}", l));
        }
        if let Some(o) = offset {
            params.push_str(&format!("&offset={}", o));
        }
        let res = self.get_with_params("/api/v1/trades/history", &params).await?;

        Ok(res)
    }
}
