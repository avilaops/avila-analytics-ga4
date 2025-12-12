//! Event types and tracking for Avila Analytics
//!
//! Supports all GA4 event types plus custom events

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Main event type
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "event_type", rename_all = "snake_case")]
pub enum Event {
    /// Page view event
    PageView {
        page_title: String,
        page_location: String,
        page_referrer: Option<String>,
        user_id: Option<String>,
        #[serde(flatten)]
        params: EventParams,
    },

    /// Custom event
    Custom {
        name: String,
        #[serde(flatten)]
        params: EventParams,
    },

    /// Click event
    Click {
        element_id: Option<String>,
        element_class: Option<String>,
        element_text: Option<String>,
        link_url: Option<String>,
        #[serde(flatten)]
        params: EventParams,
    },

    /// Form submission
    FormSubmit {
        form_id: String,
        form_name: Option<String>,
        #[serde(flatten)]
        params: EventParams,
    },

    /// E-commerce: View item
    ViewItem {
        items: Vec<Item>,
        value: Option<f64>,
        currency: Option<String>,
        #[serde(flatten)]
        params: EventParams,
    },

    /// E-commerce: Add to cart
    AddToCart {
        items: Vec<Item>,
        value: Option<f64>,
        currency: Option<String>,
        #[serde(flatten)]
        params: EventParams,
    },

    /// E-commerce: Remove from cart
    RemoveFromCart {
        items: Vec<Item>,
        value: Option<f64>,
        currency: Option<String>,
        #[serde(flatten)]
        params: EventParams,
    },

    /// E-commerce: Begin checkout
    BeginCheckout {
        items: Vec<Item>,
        value: f64,
        currency: String,
        coupon: Option<String>,
        #[serde(flatten)]
        params: EventParams,
    },

    /// E-commerce: Purchase
    Purchase {
        transaction_id: String,
        value: f64,
        currency: String,
        tax: Option<f64>,
        shipping: Option<f64>,
        items: Vec<Item>,
        coupon: Option<String>,
        #[serde(flatten)]
        params: EventParams,
    },

    /// E-commerce: Refund
    Refund {
        transaction_id: String,
        value: Option<f64>,
        currency: Option<String>,
        items: Option<Vec<Item>>,
        #[serde(flatten)]
        params: EventParams,
    },

    /// Search event
    Search {
        search_term: String,
        #[serde(flatten)]
        params: EventParams,
    },

    /// Video events
    VideoStart {
        video_title: String,
        video_url: String,
        video_provider: Option<String>,
        #[serde(flatten)]
        params: EventParams,
    },

    VideoProgress {
        video_title: String,
        video_url: String,
        video_percent: u8,
        #[serde(flatten)]
        params: EventParams,
    },

    VideoComplete {
        video_title: String,
        video_url: String,
        #[serde(flatten)]
        params: EventParams,
    },

    /// File download
    FileDownload {
        file_name: String,
        file_extension: String,
        link_url: String,
        #[serde(flatten)]
        params: EventParams,
    },

    /// Scroll tracking
    Scroll {
        percent_scrolled: u8,
        #[serde(flatten)]
        params: EventParams,
    },

    /// Session start
    SessionStart {
        #[serde(flatten)]
        params: EventParams,
    },

    /// User engagement
    UserEngagement {
        engagement_time_msec: u64,
        #[serde(flatten)]
        params: EventParams,
    },
}

/// Event parameters (common fields)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EventParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub screen_resolution: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub viewport_size: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_category: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub os: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser: Option<String>,

    /// Custom dimensions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_dimensions: Option<HashMap<String, String>>,

    /// Custom metrics
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_metrics: Option<HashMap<String, f64>>,
}

/// E-commerce item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub item_id: String,
    pub item_name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_brand: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_category: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_category2: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_variant: Option<String>,

    pub price: f64,
    pub quantity: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount: Option<f64>,
}

/// Event envelope with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventEnvelope {
    pub event_id: Uuid,
    pub measurement_id: String,
    pub timestamp: DateTime<Utc>,
    pub event: Event,
    pub processed: bool,
}

impl EventEnvelope {
    pub fn new(measurement_id: String, event: Event) -> Self {
        Self {
            event_id: Uuid::new_v4(),
            measurement_id,
            timestamp: Utc::now(),
            event,
            processed: false,
        }
    }
}

/// Batch of events for efficient processing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventBatch {
    pub batch_id: Uuid,
    pub events: Vec<EventEnvelope>,
    pub created_at: DateTime<Utc>,
}

impl EventBatch {
    pub fn new(events: Vec<EventEnvelope>) -> Self {
        Self {
            batch_id: Uuid::new_v4(),
            events,
            created_at: Utc::now(),
        }
    }

    pub fn size(&self) -> usize {
        self.events.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_serialization() {
        let event = Event::PageView {
            page_title: "Home".to_string(),
            page_location: "https://example.com".to_string(),
            page_referrer: None,
            user_id: Some("user123".to_string()),
            params: EventParams::default(),
        };

        let json = serde_json::to_string(&event).unwrap();
        assert!(json.contains("page_view"));
        assert!(json.contains("Home"));
    }

    #[test]
    fn test_event_envelope() {
        let event = Event::Custom {
            name: "test_event".to_string(),
            params: EventParams::default(),
        };

        let envelope = EventEnvelope::new("TEST123".to_string(), event);
        assert_eq!(envelope.measurement_id, "TEST123");
        assert!(!envelope.processed);
    }

    #[test]
    fn test_ecommerce_item() {
        let item = Item {
            item_id: "SKU123".to_string(),
            item_name: "Product X".to_string(),
            item_brand: Some("Brand A".to_string()),
            item_category: Some("Electronics".to_string()),
            item_category2: None,
            item_variant: Some("Blue".to_string()),
            price: 99.90,
            quantity: 2,
            coupon: None,
            discount: Some(10.0),
        };

        let json = serde_json::to_string(&item).unwrap();
        assert!(json.contains("SKU123"));
    }
}
