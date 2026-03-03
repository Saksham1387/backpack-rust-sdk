use crate::{client::BackpackClient, error::Result, types::depth::Depth};

impl BackpackClient {
    pub async fn get_depth(&self, symbol: &str, limit: Option<&str>) -> Result<Depth> {
        let params = match limit {
            Some(l) => format!("symbol={}&limit={}", symbol, l),
            None => format!("symbol={}", symbol),
        };
        let res = self.get_with_params("/api/v1/depth", &params).await?;
        Ok(res)
    }
}
