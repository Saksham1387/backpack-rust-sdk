use crate::{BackpackClient, Result, account::Account};

impl BackpackClient {
    pub async fn get_account(&self) -> Result<Account> {
        let res = self
            .signed_get("accountQuery", "/api/v1/account", "")
            .await?;

        Ok(res)
    }
}
