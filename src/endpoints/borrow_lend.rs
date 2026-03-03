use crate::{
    client::BackpackClient,
    error::Result,
    types::borrow_lend::{BorrowLendApy, BorrowLendMarket, BorrowLendMarketHistory, BorrowLendMarketHistoryInterval},
};

impl BackpackClient {
    pub async fn get_borrow_lend_markets(&self) -> Result<Vec<BorrowLendMarket>> {
        let res = self.get::<Vec<BorrowLendMarket>>("/api/v1/borrowLend/markets").await?;

        Ok(res)
    }

    pub async fn get_borrow_lend_market_history(
        &self,
        interval: BorrowLendMarketHistoryInterval,
        symbol: Option<&str>,
    ) -> Result<Vec<BorrowLendMarketHistory>> {
        let params = match symbol {
            Some(s) => format!("interval={}&symbol={}", interval, s),
            None => format!("interval={}", interval),
        };

        let res = self
            .get_with_params::<Vec<BorrowLendMarketHistory>>(
                "/api/v1/borrowLend/markets/history",
                &params,
            )
            .await?;

        Ok(res)
    }

    pub async fn get_borrow_lend_apy(&self) -> Result<BorrowLendApy> {
        let res = self.get::<BorrowLendApy>("/api/v1/borrowLend/apy").await?;

        Ok(res)
    }
}
