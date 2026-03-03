use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ticker {
    pub symbol: String,
    pub first_price: String,
    pub last_price: String,
    pub price_change: String,
    pub price_change_percent: String,
    pub high: String,
    pub low: String,
    pub volume: String,
    pub quote_volume: String,
    pub trades: String,
}
