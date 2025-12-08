//! Basic tracking example

use avila_analytics_ga4::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🚀 Avila Analytics GA4 - Basic Tracking Example\n");

    // Create analytics client
    let client = AnalyticsClient::with_endpoint(
        "G-DEMO123456",
        "http://localhost:8080",
    )?;

    println!("✅ Client created with measurement ID: G-DEMO123456\n");

    // Track page view
    println!("📄 Tracking page view...");
    client
        .track_event(Event::PageView {
            page_title: "Home Page".to_string(),
            page_location: "https://example.com/".to_string(),
            page_referrer: Some("https://google.com".to_string()),
            user_id: Some("user123".to_string()),
            params: Default::default(),
        })
        .await?;
    println!("   ✓ Page view tracked\n");

    // Track custom event
    println!("🎯 Tracking custom event...");
    client
        .track_event(Event::Custom {
            name: "button_click".to_string(),
            params: EventParams {
                custom_dimensions: Some(
                    [
                        ("button_id".to_string(), "cta_signup".to_string()),
                        ("section".to_string(), "hero".to_string()),
                    ]
                    .into_iter()
                    .collect(),
                ),
                ..Default::default()
            },
        })
        .await?;
    println!("   ✓ Custom event tracked\n");

    // Track search
    println!("🔍 Tracking search...");
    client
        .track_event(Event::Search {
            search_term: "rust analytics".to_string(),
            params: Default::default(),
        })
        .await?;
    println!("   ✓ Search tracked\n");

    // Track form submission
    println!("📝 Tracking form submission...");
    client
        .track_event(Event::FormSubmit {
            form_id: "contact_form".to_string(),
            form_name: Some("Contact Us".to_string()),
            params: Default::default(),
        })
        .await?;
    println!("   ✓ Form submission tracked\n");

    // Track file download
    println!("📥 Tracking file download...");
    client
        .track_event(Event::FileDownload {
            file_name: "product_catalog.pdf".to_string(),
            file_extension: "pdf".to_string(),
            link_url: "https://example.com/downloads/catalog.pdf".to_string(),
            params: Default::default(),
        })
        .await?;
    println!("   ✓ File download tracked\n");

    println!("✨ All events tracked successfully!");
    println!("   Check your analytics dashboard for real-time data.");

    Ok(())
}
