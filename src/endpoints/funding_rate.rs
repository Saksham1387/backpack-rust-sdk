use crate::{client::BackpackClient, error::Result, types::funding_rate::FundingRate};

impl BackpackClient {
    pub async fn get_funding_rates(
        &self,
        symbol: &str,
        limit: Option<u16>,
        offset: Option<u64>,
    ) -> Result<Vec<FundingRate>> {
        let mut params = format!("symbol={}", symbol);

        if let Some(l) = limit {
            params.push_str(&format!("&limit={}", l));
        }
        if let Some(o) = offset {
            params.push_str(&format!("&offset={}", o));
        }

        let res = self.get_with_params("/api/v1/fundingRates", &params).await?;
        Ok(res)
    }
}
