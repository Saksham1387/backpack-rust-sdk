use serde::Deserialize;

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssestToken {
    pub display_name: Option<String>,

    pub blockchain: Option<String>,

    pub contract_address: Option<String>,

    pub deposit_enabled: Option<bool>,

    pub minimum_deposit: Option<String>,

    pub withdraw_enabled: Option<bool>,

    pub minimum_withdrawal: Option<String>,
    pub maximum_withdrawal: Option<String>,

    pub withdrawal_fee: Option<String>,

    pub native_decimals: Option<u32>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Assest {
    pub symbol: Option<String>,
    pub display_name: Option<String>,
    pub coingecko_id: Option<String>,
    pub tokens: Vec<AssestToken>,
}
