//! User identification and tracking

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserIdentifier {
    pub user_id: Option<String>,
    pub client_id: String,
    pub anonymous: bool,
}

impl UserIdentifier {
    pub fn new() -> Self {
        Self {
            user_id: None,
            client_id: Uuid::new_v4().to_string(),
            anonymous: true,
        }
    }

    pub fn with_user_id(user_id: String) -> Self {
        Self {
            user_id: Some(user_id),
            client_id: Uuid::new_v4().to_string(),
            anonymous: false,
        }
    }

    pub fn set_user_id(&mut self, user_id: String) {
        self.user_id = Some(user_id);
        self.anonymous = false;
    }

    pub fn is_identified(&self) -> bool {
        self.user_id.is_some()
    }
}

impl Default for UserIdentifier {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_anonymous_user() {
        let user = UserIdentifier::new();
        assert!(user.anonymous);
        assert!(!user.is_identified());
    }

    #[test]
    fn test_identified_user() {
        let user = UserIdentifier::with_user_id("user123".to_string());
        assert!(!user.anonymous);
        assert!(user.is_identified());
    }
}
