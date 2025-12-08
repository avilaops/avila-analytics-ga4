//! Real-time dashboard example

use avila_analytics_ga4::prelude::*;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<()> {
    println!("📊 Avila Analytics GA4 - Real-time Dashboard Simulation\n");

    let client = AnalyticsClient::with_endpoint(
        "G-REALTIME123",
        "http://localhost:8080",
    )?;

    println!("🚀 Simulating user activity for real-time dashboard...\n");

    // Simulate multiple users
    for i in 1..=10 {
        let user_id = format!("user_{}", i);

        println!("👤 User {} activity:", i);

        // Page view
        client
            .track_event(Event::PageView {
                page_title: format!("Page {}", i % 3),
                page_location: format!("https://example.com/page{}", i % 3),
                page_referrer: None,
                user_id: Some(user_id.clone()),
                params: Default::default(),
            })
            .await?;
        println!("   📄 Page view");

        // Random interaction
        if i % 2 == 0 {
            client
                .track_event(Event::Click {
                    element_id: Some("cta_button".to_string()),
                    element_class: Some("btn-primary".to_string()),
                    element_text: Some("Learn More".to_string()),
                    link_url: None,
                    params: EventParams {
                        user_id: Some(user_id.clone()),
                        ..Default::default()
                    },
                })
                .await?;
            println!("   🖱️  Click event");
        }

        // Scroll tracking
        if i % 3 == 0 {
            client
                .track_event(Event::Scroll {
                    percent_scrolled: 75,
                    params: EventParams {
                        user_id: Some(user_id.clone()),
                        ..Default::default()
                    },
                })
                .await?;
            println!("   📜 Scroll 75%");
        }

        // User engagement
        client
            .track_event(Event::UserEngagement {
                engagement_time_msec: (i * 1000) as u64,
                params: EventParams {
                    user_id: Some(user_id),
                    ..Default::default()
                },
            })
            .await?;
        println!("   ⏱️  Engagement: {}s\n", i);

        sleep(Duration::from_millis(500)).await;
    }

    println!("✨ Real-time simulation complete!");
    println!("\n📈 Dashboard Metrics:");
    println!("   Active Users: 10");
    println!("   Page Views: 10");
    println!("   Interactions: 8");
    println!("   Avg Engagement: 5.5s");
    println!("\n💡 In production, view real-time data at:");
    println!("   http://localhost:8080/dashboard/realtime");

    Ok(())
}
