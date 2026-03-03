use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BorrowLendMarket {
    pub state: String,
    pub asset_mark_price: String,
    pub borrow_interest_rate: String,
    pub borrowed_quantity: String,
    pub fee: String,
    pub lend_interest_rate: String,
    pub lent_quantity: String,
    pub max_utilization: String,
    pub open_borrow_lend_limit: String,
    pub optimal_utilization: String,
    pub symbol: String,
    pub timestamp: String,
    pub throttle_utilization_threshold: String,
    pub throttle_utilization_bound: String,
    pub throttle_update_fraction: String,
    pub utilization: String,
    pub step_size: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BorrowLendMarketHistory {
    pub borrow_interest_rate: String,
    pub borrowed_quantity: String,
    pub lend_interest_rate: String,
    pub lent_quantity: String,
    pub timestamp: String,
    pub utilization: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
pub enum BorrowLendMarketHistoryInterval {
    #[serde(rename = "1d")]
    OneDay,
    #[serde(rename = "1w")]
    OneWeek,
    #[serde(rename = "1month")]
    OneMonth,
    #[serde(rename = "1year")]
    OneYear,
}

impl std::fmt::Display for BorrowLendMarketHistoryInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BorrowLendMarketHistoryInterval::OneDay => write!(f, "1d"),
            BorrowLendMarketHistoryInterval::OneWeek => write!(f, "1w"),
            BorrowLendMarketHistoryInterval::OneMonth => write!(f, "1month"),
            BorrowLendMarketHistoryInterval::OneYear => write!(f, "1year"),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BorrowLendApyItem {
    pub symbol: String,
    pub borrow_rate: String,
    pub lend_rate: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StakingApyItem {
    pub symbol: String,
    pub dilution_factor: String,
    pub staking_rate: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BorrowLendApy {
    pub borrow_lend: Vec<BorrowLendApyItem>,
    pub staking: Vec<StakingApyItem>,
}
