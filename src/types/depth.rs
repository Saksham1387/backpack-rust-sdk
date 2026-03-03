use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Depth {
    pub asks: Vec<[String; 2]>,
    pub bids: Vec<[String; 2]>,
    pub last_update_id: String,
    pub timestamp: u64,
}
