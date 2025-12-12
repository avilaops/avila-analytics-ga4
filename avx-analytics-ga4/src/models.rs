//! Data models for Avila Analytics

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Site/Property configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Site {
    pub id: Uuid,
    pub measurement_id: String,
    pub name: String,
    pub domain: String,
    pub timezone: String,
    pub currency: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub active: bool,
}

/// User profile
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub user_id: Option<String>,
    pub client_id: String,
    pub first_seen: DateTime<Utc>,
    pub last_seen: DateTime<Utc>,
    pub total_sessions: u32,
    pub total_events: u64,
    pub properties: serde_json::Value,
}

/// Session information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: Uuid,
    pub session_id: String,
    pub user_id: Uuid,
    pub site_id: Uuid,
    pub started_at: DateTime<Utc>,
    pub ended_at: Option<DateTime<Utc>>,
    pub duration_seconds: u32,
    pub page_views: u32,
    pub events_count: u32,
    pub is_bounce: bool,
    pub entry_page: String,
    pub exit_page: Option<String>,
    pub referrer: Option<String>,
    pub utm_source: Option<String>,
    pub utm_medium: Option<String>,
    pub utm_campaign: Option<String>,
    pub device_category: String,
    pub browser: String,
    pub os: String,
    pub country: Option<String>,
    pub city: Option<String>,
}

/// Page view record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageView {
    pub id: Uuid,
    pub session_id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub page_title: String,
    pub page_path: String,
    pub page_location: String,
    pub page_referrer: Option<String>,
    pub time_on_page_seconds: Option<u32>,
}

/// Conversion goal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Goal {
    pub id: Uuid,
    pub site_id: Uuid,
    pub name: String,
    pub goal_type: GoalType,
    pub value: Option<f64>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GoalType {
    Url { url_pattern: String },
    Event { event_name: String },
    Duration { min_seconds: u32 },
    PagesPerSession { min_pages: u32 },
}

/// Funnel definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Funnel {
    pub id: Uuid,
    pub site_id: Uuid,
    pub name: String,
    pub steps: Vec<FunnelStep>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunnelStep {
    pub order: u32,
    pub name: String,
    pub event_name: Option<String>,
    pub url_pattern: Option<String>,
}

/// Real-time metrics snapshot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealtimeSnapshot {
    pub timestamp: DateTime<Utc>,
    pub active_users: u32,
    pub active_sessions: u32,
    pub events_per_minute: u32,
    pub top_pages: Vec<TopPage>,
    pub top_sources: Vec<TopSource>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopPage {
    pub page_path: String,
    pub active_users: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopSource {
    pub source: String,
    pub active_users: u32,
}

/// Aggregated metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AggregatedMetrics {
    pub date: chrono::NaiveDate,
    pub site_id: Uuid,
    pub users: u32,
    pub new_users: u32,
    pub sessions: u32,
    pub page_views: u32,
    pub events: u64,
    pub bounce_rate: f64,
    pub avg_session_duration: f64,
    pub pages_per_session: f64,
    pub conversions: u32,
    pub conversion_rate: f64,
    pub revenue: Option<f64>,
}

/// Device breakdown
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceMetrics {
    pub date: chrono::NaiveDate,
    pub device_category: String,
    pub users: u32,
    pub sessions: u32,
    pub bounce_rate: f64,
    pub revenue: Option<f64>,
}

/// Traffic source
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrafficSource {
    pub source: String,
    pub medium: String,
    pub campaign: Option<String>,
    pub users: u32,
    pub sessions: u32,
    pub conversions: u32,
    pub revenue: Option<f64>,
}

/// Geographic data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeoData {
    pub country: String,
    pub city: Option<String>,
    pub users: u32,
    pub sessions: u32,
    pub revenue: Option<f64>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_site_creation() {
        let site = Site {
            id: Uuid::new_v4(),
            measurement_id: "G-XXXXXXXXXX".to_string(),
            name: "Test Site".to_string(),
            domain: "example.com".to_string(),
            timezone: "America/Sao_Paulo".to_string(),
            currency: "BRL".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            active: true,
        };

        assert_eq!(site.name, "Test Site");
        assert!(site.active);
    }
}
