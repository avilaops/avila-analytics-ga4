//! Session management

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionTracker {
    pub session_id: String,
    pub user_id: Option<String>,
    pub started_at: DateTime<Utc>,
    pub last_activity: DateTime<Utc>,
    pub page_views: u32,
    pub events_count: u32,
    pub is_active: bool,
}

impl SessionTracker {
    pub fn new(user_id: Option<String>) -> Self {
        Self {
            session_id: Uuid::new_v4().to_string(),
            user_id,
            started_at: Utc::now(),
            last_activity: Utc::now(),
            page_views: 0,
            events_count: 0,
            is_active: true,
        }
    }

    pub fn update_activity(&mut self) {
        self.last_activity = Utc::now();
        self.events_count += 1;
    }

    pub fn increment_page_views(&mut self) {
        self.page_views += 1;
        self.update_activity();
    }

    pub fn duration_seconds(&self) -> i64 {
        (self.last_activity - self.started_at).num_seconds()
    }

    pub fn is_expired(&self, timeout_minutes: i64) -> bool {
        let elapsed = Utc::now() - self.last_activity;
        elapsed.num_minutes() > timeout_minutes
    }
}

pub struct SessionManager {
    timeout_minutes: i64,
}

impl SessionManager {
    pub fn new(timeout_minutes: i64) -> Self {
        Self { timeout_minutes }
    }

    pub fn create_session(&self, user_id: Option<String>) -> SessionTracker {
        SessionTracker::new(user_id)
    }

    pub fn should_end_session(&self, session: &SessionTracker) -> bool {
        session.is_expired(self.timeout_minutes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_creation() {
        let session = SessionTracker::new(Some("user123".to_string()));
        assert!(session.is_active);
        assert_eq!(session.page_views, 0);
    }

    #[test]
    fn test_session_activity() {
        let mut session = SessionTracker::new(None);
        session.increment_page_views();
        assert_eq!(session.page_views, 1);
        assert_eq!(session.events_count, 1);
    }
}
