use crate::{client::BackpackClient, error::Result, types::collateral::Collateral};

impl BackpackClient {
    pub async fn get_collateral(&self) -> Result<Vec<Collateral>> {
        let res = self.get::<Vec<Collateral>>("/api/v1/collateral").await?;
        return Ok(res);
    }
}
