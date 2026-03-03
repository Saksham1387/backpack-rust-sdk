use crate::{client::BackpackClient, error::Result, types::assests::Assest};

impl BackpackClient {
    pub async fn get_assests(&self) -> Result<Vec<Assest>> {
        let res = self.get::<Vec<Assest>>("/api/v1/assets").await?;

        println!("{:?}", res[1]);

        return Ok(res);
    }
}
