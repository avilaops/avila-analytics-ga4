use serde::{Deserialize, Serialize};
use gloo_net::http::Request;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub data: T,
    pub success: bool,
}

pub async fn fetch_metrics() -> Result<serde_json::Value, String> {
    let response = Request::get("/api/v1/metrics")
        .send()
        .await
        .map_err(|e| e.to_string())?;

    response
        .json()
        .await
        .map_err(|e| e.to_string())
}

pub async fn fetch_realtime_users() -> Result<u32, String> {
    let data = fetch_metrics().await?;
    Ok(data["active_users"].as_u64().unwrap_or(0) as u32)
}
