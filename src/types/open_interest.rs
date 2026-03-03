use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenInterest {
    pub symbol: String,
    pub open_interest: String,
    pub timestamp: u64,
}
