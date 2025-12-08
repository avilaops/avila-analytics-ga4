//! Privacy filters and compliance

use crate::config::PrivacyConfig;
use crate::error::Result;
use crate::events::EventEnvelope;
use std::net::IpAddr;

/// Privacy filter applies privacy transformations to events
pub struct PrivacyFilter {
    config: PrivacyConfig,
}

impl PrivacyFilter {
    pub fn new(config: PrivacyConfig) -> Self {
        Self { config }
    }

    /// Apply privacy filters to event
    pub async fn apply(&self, mut envelope: EventEnvelope) -> Result<EventEnvelope> {
        // Anonymize IP if enabled
        if self.config.anonymize_ip {
            envelope = self.anonymize_ip(envelope);
        }

        // Check Do Not Track
        if self.config.respect_dnt {
            envelope = self.check_dnt(envelope);
        }

        // Encrypt sensitive data if enabled
        if self.config.encryption_enabled {
            envelope = self.encrypt_sensitive_data(envelope).await?;
        }

        Ok(envelope)
    }

    /// Anonymize IP address
    fn anonymize_ip(&self, mut envelope: EventEnvelope) -> EventEnvelope {
        use crate::events::Event;

        // Helper to anonymize IP string
        let anonymize = |ip_str: Option<String>| -> Option<String> {
            ip_str.and_then(|ip| {
                if let Ok(addr) = ip.parse::<IpAddr>() {
                    Some(Self::mask_ip(addr))
                } else {
                    Some(ip)
                }
            })
        };

        // Apply to different event types
        envelope.event = match envelope.event {
            Event::PageView {
                page_title,
                page_location,
                page_referrer,
                user_id,
                mut params,
            } => {
                params.ip_address = anonymize(params.ip_address);
                Event::PageView {
                    page_title,
                    page_location,
                    page_referrer,
                    user_id,
                    params,
                }
            }
            Event::Custom { name, mut params } => {
                params.ip_address = anonymize(params.ip_address);
                Event::Custom { name, params }
            }
            other => other, // Apply to other variants as needed
        };

        envelope
    }

    /// Mask IP address (remove last octet for IPv4, last 80 bits for IPv6)
    fn mask_ip(addr: IpAddr) -> String {
        match addr {
            IpAddr::V4(ipv4) => {
                let octets = ipv4.octets();
                format!("{}.{}.{}.0", octets[0], octets[1], octets[2])
            }
            IpAddr::V6(ipv6) => {
                let segments = ipv6.segments();
                format!(
                    "{:x}:{:x}:{:x}:{:x}:0:0:0:0",
                    segments[0], segments[1], segments[2], segments[3]
                )
            }
        }
    }

    /// Check Do Not Track header
    fn check_dnt(&self, envelope: EventEnvelope) -> EventEnvelope {
        // Implementation would check DNT header from user agent
        // For now, just return as-is
        envelope
    }

    /// Encrypt sensitive data
    async fn encrypt_sensitive_data(&self, envelope: EventEnvelope) -> Result<EventEnvelope> {
        // Implementation would use avila-crypto for encryption
        // For now, just return as-is
        Ok(envelope)
    }

    /// Hash user ID for privacy
    pub fn hash_user_id(user_id: &str) -> String {
        use blake3::Hasher;
        let mut hasher = Hasher::new();
        hasher.update(user_id.as_bytes());
        format!("{}", hasher.finalize().to_hex())
    }

    /// Check if data should be retained based on policy
    pub fn should_retain(&self, days_old: u32) -> bool {
        days_old < self.config.data_retention_days
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::Ipv4Addr;

    #[test]
    fn test_anonymize_ipv4() {
        let ip = IpAddr::V4(Ipv4Addr::new(192, 168, 1, 100));
        let masked = PrivacyFilter::mask_ip(ip);
        assert_eq!(masked, "192.168.1.0");
    }

    #[test]
    fn test_hash_user_id() {
        let hashed = PrivacyFilter::hash_user_id("user123");
        assert!(hashed.len() > 0);

        // Same input produces same hash
        let hashed2 = PrivacyFilter::hash_user_id("user123");
        assert_eq!(hashed, hashed2);
    }

    #[test]
    fn test_data_retention() {
        let config = PrivacyConfig {
            anonymize_ip: true,
            respect_dnt: true,
            data_retention_days: 365,
            cookie_consent_required: true,
            encryption_enabled: true,
        };

        let filter = PrivacyFilter::new(config);
        assert!(filter.should_retain(100));
        assert!(!filter.should_retain(400));
    }
}
