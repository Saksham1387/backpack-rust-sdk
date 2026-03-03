#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    pub auto_borrow_settlements: bool,
    pub auto_lend: bool,
    pub auto_realize_pnl: bool,
    pub auto_repay_borrows: bool,
    pub borrow_limit: String,
    pub futures_maker_fee: String,
    pub leverage_limit: String,
    pub limit_orders: u64,
    pub liquidating: bool,
    pub position_limit: String,
    pub spot_maker_fee: String,
    pub spot_taker_fee: String,
    pub trigger_orders: u64,
}
