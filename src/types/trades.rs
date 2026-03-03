use serde::Deserialize;
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Trade {
    pub id: u64,
    pub price: String,
    pub quantity: String,
    pub quote_quantity: String,
    pub timestamp: u64,
    pub is_buyer_maker: bool,
}
