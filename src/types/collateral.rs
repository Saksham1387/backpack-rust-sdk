#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImfFunction {
    pub r#type: String,
    pub base: String,
    pub factor: String,
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Kind {
    #[serde(rename = "type")]
    pub kind_type: String,
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HaircutFunction {
    pub weight: String,
    pub kind: Kind,
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Collateral {
    pub symbol: String,
    pub imf_function: ImfFunction,
    pub mmf_function: ImfFunction,
    pub haircut_function: HaircutFunction,
}
