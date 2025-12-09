# 🦀 Guia de Integração - Rust SDK

> **Para aplicações backend, APIs e serviços em Rust**

---

## 📋 Pré-requisitos

- ✅ Rust 1.75+
- ✅ Tokio async runtime
- ✅ Measurement ID fornecido pela equipe Analytics

---

## 🚀 Quick Start

### Passo 1: Adicionar Dependência

No `Cargo.toml`:

```toml
[dependencies]
avila-analytics-ga4 = { path = "../avila-analytics-ga4" }
# ou quando publicado no crates.io:
# avila-analytics-ga4 = "0.1"

tokio = { version = "1.35", features = ["full"] }
```

### Passo 2: Inicializar Client

```rust
use avila_analytics_ga4::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    // Criar client
    let analytics = AnalyticsClient::new("G-XXXXXXXXXX").await?;
    
    // Track event
    analytics.track_event(Event::PageView {
        page_title: "Home".into(),
        page_location: "https://example.com".into(),
        user_id: Some("user123".into()),
    }).await?;
    
    Ok(())
}
```

### Passo 3: Verificar Dashboard

Acesse: `https://analytics.avilaops.com/dashboard?site=G-XXXXXXXXXX`

---

## 🎯 Event Types

### 1. Page View

```rust
analytics.track_event(Event::PageView {
    page_title: "Home".into(),
    page_location: "https://example.com".into(),
    user_id: Some("user123".into()),
}).await?;
```

### 2. Custom Event

```rust
use std::collections::HashMap;

let mut params = HashMap::new();
params.insert("button_id".to_string(), "cta_main".to_string());
params.insert("section".to_string(), "hero".to_string());

analytics.track_event(Event::Custom {
    name: "button_click".into(),
    params,
}).await?;
```

### 3. User Event

```rust
analytics.track_event(Event::User {
    event_type: UserEventType::Login,
    user_id: "user123".into(),
    properties: hashmap! {
        "method" => "email",
        "provider" => "google",
    },
}).await?;
```

### 4. E-commerce Events

```rust
// Product view
analytics.track_event(Event::ViewItem {
    items: vec![
        Item {
            item_id: "SKU123".into(),
            item_name: "Produto X".into(),
            price: 99.90,
            quantity: 1,
            category: Some("Eletrônicos".into()),
            brand: Some("Marca Y".into()),
        }
    ],
}).await?;

// Add to cart
analytics.track_event(Event::AddToCart {
    items: vec![
        Item {
            item_id: "SKU123".into(),
            item_name: "Produto X".into(),
            price: 99.90,
            quantity: 1,
            category: None,
            brand: None,
        }
    ],
    value: 99.90,
    currency: "BRL".into(),
}).await?;

// Purchase
analytics.track_event(Event::Purchase {
    transaction_id: "TXN123".into(),
    value: 199.80,
    currency: "BRL".into(),
    tax: Some(10.00),
    shipping: Some(15.00),
    items: vec![
        Item {
            item_id: "SKU123".into(),
            item_name: "Produto X".into(),
            price: 99.90,
            quantity: 2,
            category: Some("Eletrônicos".into()),
            brand: Some("Marca Y".into()),
        }
    ],
}).await?;
```

---

## 🏗️ Padrões de Integração

### 1. Singleton Global

Para uso em toda a aplicação:

```rust
use once_cell::sync::Lazy;
use avila_analytics_ga4::prelude::*;

// Global singleton
static ANALYTICS: Lazy<AnalyticsClient> = Lazy::new(|| {
    AnalyticsClient::new("G-XXXXXXXXXX")
        .expect("Failed to initialize analytics")
});

// Uso em qualquer lugar
async fn my_function() {
    ANALYTICS.track_event(Event::Custom {
        name: "function_called".into(),
        params: hashmap! {
            "function" => "my_function",
        },
    }).await.ok();
}
```

### 2. Dependency Injection

Para testes e modularidade:

```rust
use avila_analytics_ga4::prelude::*;

struct AppState {
    analytics: AnalyticsClient,
}

impl AppState {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            analytics: AnalyticsClient::new("G-XXXXXXXXXX").await?,
        })
    }
}

// Em handlers
async fn handle_request(state: &AppState) -> Result<()> {
    state.analytics.track_event(Event::Custom {
        name: "api_call".into(),
        params: hashmap! {
            "endpoint" => "/api/users",
        },
    }).await?;
    
    Ok(())
}
```

### 3. Axum Integration

Para APIs REST com Axum:

```rust
use axum::{
    Router,
    extract::State,
    routing::get,
    Json,
};
use avila_analytics_ga4::prelude::*;

#[derive(Clone)]
struct AppState {
    analytics: AnalyticsClient,
}

async fn api_handler(
    State(state): State<AppState>,
) -> Json<String> {
    // Track API call
    state.analytics.track_event(Event::Custom {
        name: "api_call".into(),
        params: hashmap! {
            "endpoint" => "/api/data",
            "method" => "GET",
        },
    }).await.ok();
    
    Json("OK".into())
}

#[tokio::main]
async fn main() -> Result<()> {
    let state = AppState {
        analytics: AnalyticsClient::new("G-XXXXXXXXXX").await?,
    };
    
    let app = Router::new()
        .route("/api/data", get(api_handler))
        .with_state(state);
    
    axum::Server::bind(&"0.0.0.0:3000".parse()?)
        .serve(app.into_make_service())
        .await?;
    
    Ok(())
}
```

---

## 📊 Use Cases por Tipo de Aplicação

### Backend API

```rust
use axum::{Router, routing::post, extract::Json, Extension};
use avila_analytics_ga4::prelude::*;

async fn create_user(
    Extension(analytics): Extension<AnalyticsClient>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<Json<User>> {
    // Criar usuário
    let user = User::create(payload).await?;
    
    // Track evento
    analytics.track_event(Event::Custom {
        name: "user_created".into(),
        params: hashmap! {
            "user_id" => user.id.to_string(),
            "method" => "api",
        },
    }).await.ok();
    
    Ok(Json(user))
}
```

### Background Jobs

```rust
use avila_analytics_ga4::prelude::*;

async fn process_job(job: Job, analytics: &AnalyticsClient) -> Result<()> {
    let start = std::time::Instant::now();
    
    // Processar job
    let result = job.process().await;
    
    let duration = start.elapsed().as_secs_f64();
    
    // Track resultado
    analytics.track_event(Event::Custom {
        name: "job_processed".into(),
        params: hashmap! {
            "job_id" => job.id.to_string(),
            "job_type" => job.job_type.clone(),
            "status" => if result.is_ok() { "success" } else { "failed" },
            "duration_secs" => duration.to_string(),
        },
    }).await.ok();
    
    result
}
```

### CLI Tools

```rust
use avila_analytics_ga4::prelude::*;
use clap::Parser;

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    command: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let analytics = AnalyticsClient::new("G-CLI-01").await?;
    
    // Track comando executado
    analytics.track_event(Event::Custom {
        name: "cli_command".into(),
        params: hashmap! {
            "command" => cli.command.clone(),
        },
    }).await.ok();
    
    // Executar comando
    execute_command(&cli.command).await?;
    
    Ok(())
}
```

### Microservices

```rust
use avila_analytics_ga4::prelude::*;

struct OrderService {
    analytics: AnalyticsClient,
}

impl OrderService {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            analytics: AnalyticsClient::new("G-ORDERS-01").await?,
        })
    }
    
    pub async fn create_order(&self, order: Order) -> Result<Order> {
        // Criar pedido
        let created_order = self.save_order(order).await?;
        
        // Track evento
        self.analytics.track_event(Event::Purchase {
            transaction_id: created_order.id.clone(),
            value: created_order.total,
            currency: "BRL".into(),
            tax: created_order.tax,
            shipping: created_order.shipping,
            items: created_order.items.clone(),
        }).await.ok();
        
        Ok(created_order)
    }
}
```

---

## 🔧 Configuração Avançada

### Custom Endpoint

```rust
let analytics = AnalyticsClient::builder()
    .measurement_id("G-XXXXXXXXXX")
    .endpoint("https://custom-analytics.com/api/v1/collect")
    .build()
    .await?;
```

### Batch Mode

Para alta performance com muitos eventos:

```rust
let analytics = AnalyticsClient::builder()
    .measurement_id("G-XXXXXXXXXX")
    .batch_size(100)  // Enviar a cada 100 eventos
    .flush_interval(5)  // ou a cada 5 segundos
    .build()
    .await?;

// Events são batched automaticamente
for i in 0..1000 {
    analytics.track_event(Event::Custom {
        name: "test_event".into(),
        params: hashmap! {
            "index" => i.to_string(),
        },
    }).await?;
}

// Forçar flush
analytics.flush().await?;
```

### Error Handling

```rust
match analytics.track_event(event).await {
    Ok(_) => println!("Event tracked successfully"),
    Err(e) => {
        eprintln!("Failed to track event: {}", e);
        // Não bloquear aplicação por falha de analytics
    }
}

// Ou simplesmente ignore erros
analytics.track_event(event).await.ok();
```

### Timeout Configuration

```rust
let analytics = AnalyticsClient::builder()
    .measurement_id("G-XXXXXXXXXX")
    .timeout(std::time::Duration::from_secs(5))
    .build()
    .await?;
```

---

## 🧪 Testing

### Mock Client

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::*;
    
    #[tokio::test]
    async fn test_user_creation_tracks_event() {
        let mut mock_analytics = MockAnalyticsClient::new();
        
        mock_analytics
            .expect_track_event()
            .with(predicate::eq(Event::Custom {
                name: "user_created".into(),
                params: hashmap! {
                    "method" => "test",
                },
            }))
            .times(1)
            .returning(|_| Ok(()));
        
        // Test code using mock_analytics
    }
}
```

### Integration Tests

```rust
#[tokio::test]
#[ignore]  // Rodar apenas em CI com analytics server
async fn test_analytics_integration() -> Result<()> {
    let analytics = AnalyticsClient::new("G-TEST-01").await?;
    
    analytics.track_event(Event::Custom {
        name: "integration_test".into(),
        params: hashmap! {
            "test_id" => "test_123",
        },
    }).await?;
    
    // Verificar no dashboard ou API
    // ...
    
    Ok(())
}
```

---

## ⚡ Performance Tips

### 1. Use Lazy Initialization

```rust
use once_cell::sync::Lazy;

static ANALYTICS: Lazy<AnalyticsClient> = Lazy::new(|| {
    AnalyticsClient::new("G-XXXXXXXXXX").expect("Failed to init")
});
```

### 2. Fire-and-Forget

Para não bloquear requisições:

```rust
tokio::spawn(async move {
    analytics.track_event(event).await.ok();
});
```

### 3. Batch Events

```rust
// Coletar eventos
let events = vec![event1, event2, event3];

// Enviar em batch
analytics.track_batch(events).await?;
```

### 4. Async Processing

```rust
use tokio::sync::mpsc;

// Canal para eventos
let (tx, mut rx) = mpsc::channel(1000);

// Worker task
tokio::spawn(async move {
    while let Some(event) = rx.recv().await {
        ANALYTICS.track_event(event).await.ok();
    }
});

// Enviar eventos
tx.send(event).await?;
```

---

## 📊 Métricas Internas

O SDK expõe métricas Prometheus:

```rust
use prometheus::{Registry, Encoder, TextEncoder};

// Registrar métricas do SDK
let registry = Registry::new();
analytics.register_metrics(&registry)?;

// Endpoint de métricas
async fn metrics_handler() -> String {
    let encoder = TextEncoder::new();
    let metric_families = registry.gather();
    
    let mut buffer = vec![];
    encoder.encode(&metric_families, &mut buffer).unwrap();
    String::from_utf8(buffer).unwrap()
}
```

Métricas disponíveis:
- `avila_analytics_events_sent_total`
- `avila_analytics_events_failed_total`
- `avila_analytics_api_duration_seconds`

---

## 🔒 Privacy & Security

### IP Anonymization

Por padrão, IPs são anonimizados no servidor. Para controle no client:

```rust
let analytics = AnalyticsClient::builder()
    .measurement_id("G-XXXXXXXXXX")
    .anonymize_ip(true)
    .build()
    .await?;
```

### User ID Hashing

```rust
use blake3;

fn hash_user_id(user_id: &str) -> String {
    let hash = blake3::hash(user_id.as_bytes());
    hash.to_hex().to_string()
}

analytics.track_event(Event::User {
    event_type: UserEventType::Login,
    user_id: hash_user_id("user@example.com"),
    properties: HashMap::new(),
}).await?;
```

---

## 📚 Exemplos Completos

### Exemplo 1: REST API com Axum

```rust
use axum::{Router, routing::{get, post}, extract::State, Json};
use avila_analytics_ga4::prelude::*;
use std::sync::Arc;

#[derive(Clone)]
struct AppState {
    analytics: Arc<AnalyticsClient>,
}

async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<Json<User>> {
    let user = User::create(payload).await?;
    
    state.analytics.track_event(Event::Custom {
        name: "user_created".into(),
        params: hashmap! {
            "user_id" => user.id.to_string(),
        },
    }).await.ok();
    
    Ok(Json(user))
}

async fn health_check() -> &'static str {
    "OK"
}

#[tokio::main]
async fn main() -> Result<()> {
    let analytics = Arc::new(
        AnalyticsClient::new("G-API-01").await?
    );
    
    let state = AppState { analytics };
    
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/users", post(create_user))
        .with_state(state);
    
    axum::Server::bind(&"0.0.0.0:3000".parse()?)
        .serve(app.into_make_service())
        .await?;
    
    Ok(())
}
```

### Exemplo 2: E-commerce Backend

Ver: `examples/ecommerce_tracking.rs`

### Exemplo 3: CLI Tool

Ver: `examples/basic_tracking.rs`

---

## ✅ Checklist de Implementação

- [ ] Dependência adicionada ao Cargo.toml
- [ ] Client inicializado com Measurement ID correto
- [ ] Events principais tracked
- [ ] Error handling implementado
- [ ] Testes escritos
- [ ] Performance otimizada (batch, async)
- [ ] Privacy compliant (hash de user IDs)
- [ ] Documentação atualizada

---

## 🆘 Suporte

**Problemas ou dúvidas?**

- 📧 Email: analytics@avilaops.com
- 💬 Slack: #analytics
- 📚 Docs: https://docs.avilaops.com/analytics
- 🐛 Issues: https://github.com/avilaops/avila-analytics-ga4/issues

---

**Última atualização:** Dezembro 2025  
**Versão:** 1.0
