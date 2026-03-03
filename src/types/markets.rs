use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Market {
    pub symbol: String,
    pub base_symbol: String,
    pub quote_symbol: String,
    pub market_type: String,
    pub filters: Filters,
    pub imf_function: ImfFunction,
    pub mmf_function: MmfFunction,
    pub funding_interval: u64,
    pub funding_rate_upper_bound: String,
    pub funding_rate_lower_bound: String,
    pub open_interest_limit: String,
    pub order_book_state: String,
    pub created_at: String,
    pub visible: bool,
    pub position_limit_weight: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Filters {
    pub price: PriceFilter,
    pub quantity: QuantityFilter,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PriceFilter {
    pub min_price: String,
    pub max_price: String,
    pub tick_size: String,
    pub max_multiplier: String,
    pub min_multiplier: String,
    pub max_impact_multiplier: String,
    pub min_impact_multiplier: String,
    pub mean_mark_price_band: MeanMarkPriceBand,
    pub mean_premium_band: MeanPremiumBand,
    pub borrow_entry_fee_max_multiplier: String,
    pub borrow_entry_fee_min_multiplier: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MeanMarkPriceBand {
    pub max_multiplier: String,
    pub min_multiplier: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MeanPremiumBand {
    pub tolerance_pct: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuantityFilter {
    pub min_quantity: String,
    pub max_quantity: String,
    pub step_size: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImfFunction {
    #[serde(rename = "type")]
    pub type_: String,
    pub base: String,
    pub factor: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MmfFunction {
    #[serde(rename = "type")]
    pub type_: String,
    pub base: String,
    pub factor: String,
}
