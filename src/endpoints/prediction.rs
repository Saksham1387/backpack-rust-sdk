use crate::{client::BackpackClient, error::Result, types::prediction::PredictionEvent};

impl BackpackClient {
    pub async fn get_prediction_events(
        &self,
        symbol: Option<&str>,
        tag_slug: Option<&str>,
        event_slug: Option<&str>,
        series_slug: Option<&str>,
        resolved: Option<bool>,
        limit: Option<u16>,
        offset: Option<u64>,
    ) -> Result<Vec<PredictionEvent>> {
        let mut params = String::new();

        if let Some(s) = symbol {
            params.push_str(&format!("symbol={}&", s));
        }
        if let Some(t) = tag_slug {
            params.push_str(&format!("tagSlug={}&", t));
        }
        if let Some(e) = event_slug {
            params.push_str(&format!("eventSlug={}&", e));
        }
        if let Some(s) = series_slug {
            params.push_str(&format!("seriesSlug={}&", s));
        }
        if let Some(r) = resolved {
            params.push_str(&format!("resolved={}&", r));
        }
        if let Some(l) = limit {
            params.push_str(&format!("limit={}&", l));
        }
        if let Some(o) = offset {
            params.push_str(&format!("offset={}", o));
        }

        let params = params.trim_end_matches('&').to_string();

        if params.is_empty() {
            let res = self.get::<Vec<PredictionEvent>>("/api/v1/prediction").await?;
            Ok(res)
        } else {
            let res = self.get_with_params("/api/v1/prediction", &params).await?;
            Ok(res)
        }
    }
}
