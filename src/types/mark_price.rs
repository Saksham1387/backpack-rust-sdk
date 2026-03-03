use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarkPrice {
    pub funding_rate: String,
    pub index_price: String,
    pub mark_price: String,
    pub next_funding_timestamp: u64,
    pub symbol: String,
}
