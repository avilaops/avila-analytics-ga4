use yew::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Event {
    pub event_type: String,
    pub page: String,
    pub timestamp: String,
    pub user_id: Option<String>,
}

#[function_component(EventList)]
pub fn event_list() -> Html {
    let events = use_state(|| vec![
        Event {
            event_type: "page_view".to_string(),
            page: "/home".to_string(),
            timestamp: "2s ago".to_string(),
            user_id: Some("user_123".to_string()),
        },
        Event {
            event_type: "click".to_string(),
            page: "/products".to_string(),
            timestamp: "5s ago".to_string(),
            user_id: Some("user_456".to_string()),
        },
        Event {
            event_type: "purchase".to_string(),
            page: "/checkout".to_string(),
            timestamp: "12s ago".to_string(),
            user_id: Some("user_789".to_string()),
        },
    ]);

    html! {
        <div class="event-list-container">
            <h3 class="list-title">{"Recent Events"}</h3>
            <div class="event-list">
                { for events.iter().map(|event| html! {
                    <div class="event-item">
                        <div class="event-type">{get_event_icon(&event.event_type)}</div>
                        <div class="event-details">
                            <div class="event-name">{&event.event_type}</div>
                            <div class="event-page">{&event.page}</div>
                        </div>
                        <div class="event-time">{&event.timestamp}</div>
                    </div>
                })}
            </div>
        </div>
    }
}

fn get_event_icon(event_type: &str) -> &'static str {
    match event_type {
        "page_view" => "📄",
        "click" => "🖱️",
        "purchase" => "🛒",
        "form_submit" => "📝",
        "video_play" => "▶️",
        _ => "📊",
    }
}
