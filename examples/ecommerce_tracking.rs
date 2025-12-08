//! E-commerce tracking example

use avila_analytics_ga4::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🛍️  Avila Analytics GA4 - E-commerce Tracking Example\n");

    let client = AnalyticsClient::with_endpoint(
        "G-ECOMMERCE123",
        "http://localhost:8080",
    )?;

    // Sample products
    let product1 = Item {
        item_id: "SKU001".to_string(),
        item_name: "Mechanical Keyboard".to_string(),
        item_brand: Some("TechPro".to_string()),
        item_category: Some("Electronics".to_string()),
        item_category2: Some("Computer Accessories".to_string()),
        item_variant: Some("Cherry MX Blue".to_string()),
        price: 299.90,
        quantity: 1,
        coupon: None,
        discount: Some(30.0),
    };

    let product2 = Item {
        item_id: "SKU002".to_string(),
        item_name: "Gaming Mouse".to_string(),
        item_brand: Some("TechPro".to_string()),
        item_category: Some("Electronics".to_string()),
        item_category2: Some("Computer Accessories".to_string()),
        item_variant: Some("RGB Black".to_string()),
        price: 149.90,
        quantity: 1,
        coupon: None,
        discount: Some(15.0),
    };

    // 1. View item
    println!("👀 User views product...");
    client
        .track_event(Event::ViewItem {
            items: vec![product1.clone()],
            value: Some(299.90),
            currency: Some("BRL".to_string()),
            params: EventParams {
                user_id: Some("customer_456".to_string()),
                ..Default::default()
            },
        })
        .await?;
    println!("   ✓ Product view tracked\n");

    // 2. Add to cart
    println!("🛒 User adds product to cart...");
    client
        .track_event(Event::AddToCart {
            items: vec![product1.clone()],
            value: Some(299.90),
            currency: Some("BRL".to_string()),
            params: EventParams {
                user_id: Some("customer_456".to_string()),
                ..Default::default()
            },
        })
        .await?;
    println!("   ✓ Add to cart tracked\n");

    // 3. Add second item
    println!("🛒 User adds another product...");
    client
        .track_event(Event::AddToCart {
            items: vec![product2.clone()],
            value: Some(149.90),
            currency: Some("BRL".to_string()),
            params: EventParams {
                user_id: Some("customer_456".to_string()),
                ..Default::default()
            },
        })
        .await?;
    println!("   ✓ Second item added\n");

    // 4. Begin checkout
    println!("💳 User begins checkout...");
    client
        .track_event(Event::BeginCheckout {
            items: vec![product1.clone(), product2.clone()],
            value: 449.80,
            currency: "BRL".to_string(),
            coupon: Some("FIRSTBUY10".to_string()),
            params: EventParams {
                user_id: Some("customer_456".to_string()),
                ..Default::default()
            },
        })
        .await?;
    println!("   ✓ Checkout started\n");

    // 5. Complete purchase
    println!("✅ Purchase completed!");
    client
        .track_event(Event::Purchase {
            transaction_id: "TXN-2024-001234".to_string(),
            value: 404.82, // After 10% discount coupon
            currency: "BRL".to_string(),
            tax: Some(20.24),
            shipping: Some(15.00),
            items: vec![product1.clone(), product2.clone()],
            coupon: Some("FIRSTBUY10".to_string()),
            params: EventParams {
                user_id: Some("customer_456".to_string()),
                ..Default::default()
            },
        })
        .await?;
    println!("   ✓ Purchase tracked\n");

    println!("📊 Summary:");
    println!("   Transaction ID: TXN-2024-001234");
    println!("   Items: 2");
    println!("   Subtotal: R$ 449.80");
    println!("   Discount: R$ 44.98 (10% coupon)");
    println!("   Shipping: R$ 15.00");
    println!("   Tax: R$ 20.24");
    println!("   Total: R$ 404.82");

    println!("\n✨ E-commerce tracking complete!");

    Ok(())
}
