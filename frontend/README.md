# 🚀 Avila Analytics GA4 - Frontend

## Stack

- **Dashboard**: Rust + Yew + WebAssembly
- **Tracker**: Vanilla JavaScript (< 5KB minified)
- **Styling**: Pure CSS (no frameworks)

## Estrutura

```
frontend/
├── wasm-dashboard/          # Dashboard em Rust/WASM
│   ├── src/
│   │   ├── components/      # Componentes Yew
│   │   ├── api/             # API client
│   │   ├── charts/          # Gráficos Canvas
│   │   └── utils/           # Helpers
│   └── Cargo.toml
├── tracker/                 # JavaScript tracker
│   └── avila-tracker.js     # Tracker snippet
└── static/                  # Arquivos estáticos
    ├── index.html           # Dashboard HTML
    ├── demo.html            # Demo page
    ├── quickstart.html      # Quick start guide
    └── styles.css           # Global styles
```

## Build

### Pré-requisitos

```bash
# Install wasm-pack
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# Install Node.js (opcional, para minificação)
# https://nodejs.org/
```

### Build Frontend

**Linux/Mac:**
```bash
chmod +x build-frontend.sh
./build-frontend.sh
```

**Windows:**
```powershell
.\build-frontend.ps1
```

**Manual:**
```bash
# Build WASM dashboard
cd frontend/wasm-dashboard
wasm-pack build --target web --out-dir ../../static/pkg

# Minify tracker (opcional)
npx terser frontend/tracker/avila-tracker.js -c -m -o frontend/static/avila-tracker.min.js
```

## Desenvolvimento

### Servidor de desenvolvimento

```bash
# Terminal 1: Backend
cargo run --bin avila-analytics

# Terminal 2: Frontend
cd frontend/static
python3 -m http.server 8000
```

Acesse: http://localhost:8000

## Uso do Tracker

### Instalação Básica

```html
<!-- Add antes do </body> -->
<script src="http://localhost:8080/avila-tracker.js"
        data-site="G-XXXXXXXXXX"></script>
```

### Configuração

```html
<script src="avila-tracker.js"
        data-site="G-XXXXXXXXXX"
        data-debug="true"
        data-auto-track="true"></script>
```

**Opções:**
- `data-site`: Measurement ID (obrigatório)
- `data-debug`: Habilita logs no console
- `data-auto-track`: Auto-tracking de eventos (default: true)

### API JavaScript

```javascript
// Track custom event
avila.track('button_click', {
    button_id: 'cta-main',
    section: 'hero'
});

// Track page view manualmente
avila.trackPageView();

// E-commerce tracking
avila.trackEcommerce('purchase', {
    transaction_id: 'TXN123',
    value: 99.99,
    currency: 'USD',
    items: [{
        item_id: 'SKU001',
        item_name: 'Product X',
        price: 99.99,
        quantity: 1
    }]
});

// Set user ID
avila.setUserId('user_12345');

// Get user ID
const userId = avila.getUserId();
```

## Auto-tracking

O tracker automaticamente rastreia:

- ✅ **Page views**: Ao carregar a página
- ✅ **Clicks**: Em links e botões
- ✅ **Form submits**: Envio de formulários
- ✅ **File downloads**: PDFs, ZIPs, etc
- ✅ **Scroll depth**: 25%, 50%, 75%, 90%

## Dashboard Components

### MetricCard
```rust
<MetricCard
    title="Active Users"
    value="1,234"
    change={12.5}
    icon="👥"
/>
```

### RealtimeChart
```rust
<RealtimeChart />
```

### EventList
```rust
<EventList />
```

## Performance

- **Tracker size**: ~4KB minified
- **Dashboard WASM**: ~200KB gzipped
- **First paint**: < 100ms
- **Event latency**: < 10ms

## Privacy

- IP anonymization automática
- Respeita Do Not Track (DNT)
- Sem cookies de terceiros
- GDPR/LGPD compliant

## Exemplo Completo

```html
<!DOCTYPE html>
<html>
<head>
    <title>My Website</title>
</head>
<body>
    <h1>Welcome</h1>
    <button id="cta">Click Me</button>

    <!-- Avila Analytics -->
    <script src="http://localhost:8080/avila-tracker.js"
            data-site="G-DEMO123456"
            data-debug="true"></script>

    <script>
        // Custom tracking
        document.getElementById('cta').addEventListener('click', () => {
            avila.track('cta_click', {
                button_id: 'cta',
                page: 'home'
            });
        });
    </script>
</body>
</html>
```

## Deploy

### Servir arquivos estáticos

```bash
# Python
cd frontend/static && python3 -m http.server 8000

# Node.js (http-server)
npx http-server frontend/static -p 8000

# Nginx
# Copie frontend/static/* para /var/www/html/
```

### CDN

Para produção, sirva o tracker via CDN:

```html
<script src="https://cdn.yoursite.com/avila-tracker.min.js"
        data-site="G-PROD123456"></script>
```

## Troubleshooting

### WASM não carrega

Certifique-se que o servidor serve arquivos `.wasm` com MIME type correto:

**Nginx:**
```nginx
types {
    application/wasm wasm;
}
```

**Python:**
```python
import mimetypes
mimetypes.add_type('application/wasm', '.wasm')
```

### CORS errors

Configure o backend para aceitar requisições do frontend:

```rust
// No server.rs, já está configurado com CorsLayer::permissive()
```

## Roadmap

- [ ] Dark mode
- [ ] Mobile-optimized dashboard
- [ ] Advanced charts (heatmaps, funnels)
- [ ] Export reports (PDF, CSV)
- [ ] Real-time alerts
- [ ] A/B testing UI

## License

MIT
