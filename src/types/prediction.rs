use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PredictionEvent {
    pub slug: String,
    pub title: String,
    pub prediction_markets: Vec<serde_json::Value>,
    pub tags: Vec<String>,
    pub series: Vec<serde_json::Value>,
    pub description: String,
    pub img_url: String,
    pub quote_volume: String,
    pub resolution: serde_json::Value,
    pub estimated_end_date: String,
    pub resolved: bool,
}
