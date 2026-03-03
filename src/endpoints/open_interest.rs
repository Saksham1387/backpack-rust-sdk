use crate::{client::BackpackClient, error::Result, types::open_interest::OpenInterest};

impl BackpackClient {
    pub async fn get_open_interest(&self, symbol: Option<&str>) -> Result<Vec<OpenInterest>> {
        let params = match symbol {
            Some(s) => format!("symbol={}", s),
            None => String::new(),
        };

        if params.is_empty() {
            let res = self.get::<Vec<OpenInterest>>("/api/v1/openInterest").await?;
            Ok(res)
        } else {
            let res = self.get_with_params("/api/v1/openInterest", &params).await?;
            Ok(res)
        }
    }
}
