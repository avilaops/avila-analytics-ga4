# 🌐 Guia de Integração - Websites HTML/JavaScript

> **Para sites institucionais, landing pages e páginas estáticas**

---

## 📋 Pré-requisitos

- ✅ Acesso ao código HTML do site
- ✅ Measurement ID fornecido pela equipe Analytics
- ✅ 5-15 minutos de tempo

---

## 🚀 Quick Start

### Passo 1: Adicionar o Script

Adicione este código **antes do fechamento da tag `</body>`**:

```html
<!-- Avila Analytics Tracking -->
<script src="https://analytics.avilaops.com/avila-tracker.js" 
        data-site="SEU_MEASUREMENT_ID"></script>
```

**Substitua `SEU_MEASUREMENT_ID`** pelo ID fornecido pela equipe (ex: `G-AVILAOPS-01`)

### Passo 2: Testar

1. Abra o site no navegador
2. Abra o Console do DevTools (F12)
3. Verifique se aparece: `✅ Avila Analytics initialized`
4. Navegue entre páginas e veja os events sendo tracked

### Passo 3: Verificar Dashboard

Acesse: `https://analytics.avilaops.com/dashboard?site=SEU_MEASUREMENT_ID`

Você deve ver:
- ✅ Pageviews aparecendo
- ✅ Active users
- ✅ Real-time metrics

---

## ✨ Features Automáticas

### Auto-tracking (Zero Config)

O tracker automaticamente coleta:

#### 1. Page Views
```javascript
// Automaticamente tracked em cada page load
{
    event: 'page_view',
    page_title: 'Home - AvilaOps',
    page_location: 'https://avilaops.com/',
    referrer: 'https://google.com/search?q=...'
}
```

#### 2. Click Events
```javascript
// Tracked automaticamente para:
// - Links externos
// - Links de download
// - Botões com class="btn" ou role="button"
{
    event: 'click',
    element_type: 'link',
    element_text: 'Saiba Mais',
    element_url: 'https://avilaops.com/sobre'
}
```

#### 3. Form Submissions
```javascript
// Tracked quando formulários são enviados
{
    event: 'form_submit',
    form_id: 'contact-form',
    form_name: 'Contato'
}
```

#### 4. Scroll Depth
```javascript
// Tracked automaticamente em 25%, 50%, 75%, 90%
{
    event: 'scroll',
    scroll_depth: 50,  // 50% da página
    page_title: 'Sobre'
}
```

#### 5. File Downloads
```javascript
// Tracked para links de PDF, DOC, ZIP, etc
{
    event: 'file_download',
    file_name: 'catalogo.pdf',
    file_url: 'https://avilaops.com/files/catalogo.pdf'
}
```

---

## 🎯 Custom Events

### Track Eventos Customizados

```html
<button onclick="avila.track('button_click', {
    button_id: 'cta_hero',
    button_text: 'Solicitar Orçamento'
})">
    Solicitar Orçamento
</button>
```

### Exemplos Práticos

#### Botão CTA
```html
<button id="cta-contact" class="btn btn-primary">
    Entre em Contato
</button>

<script>
document.getElementById('cta-contact').addEventListener('click', function() {
    avila.track('cta_clicked', {
        cta_id: 'contact',
        cta_location: 'hero_section'
    });
});
</script>
```

#### Link de WhatsApp
```html
<a href="https://wa.me/5511999999999" 
   onclick="avila.track('whatsapp_click', {
       source: 'header',
       phone: '5511999999999'
   })">
    Fale Conosco
</a>
```

#### Vídeo Play
```html
<video id="intro-video">
    <source src="intro.mp4" type="video/mp4">
</video>

<script>
document.getElementById('intro-video').addEventListener('play', function() {
    avila.track('video_play', {
        video_name: 'intro',
        video_duration: this.duration
    });
});
</script>
```

#### Modal Aberto
```html
<script>
// Quando modal é aberto
$('#myModal').on('shown.bs.modal', function() {
    avila.track('modal_opened', {
        modal_id: 'contact_modal',
        modal_title: 'Entre em Contato'
    });
});
</script>
```

---

## 🔧 Configuração Avançada

### Opções do Data Attributes

```html
<script src="https://analytics.avilaops.com/avila-tracker.js"
        data-site="G-AVILAOPS-01"
        data-auto-track="true"
        data-respect-dnt="true"
        data-cookie-domain=".avilaops.com"
        data-debug="false">
</script>
```

**Atributos disponíveis:**

| Atributo | Valor Padrão | Descrição |
|----------|--------------|-----------|
| `data-site` | *obrigatório* | Measurement ID |
| `data-auto-track` | `true` | Auto-tracking de pageviews |
| `data-respect-dnt` | `true` | Respeitar "Do Not Track" |
| `data-cookie-domain` | `auto` | Domínio dos cookies |
| `data-debug` | `false` | Modo debug (console logs) |

### Modo Debug

Para debug:

```html
<script src="https://analytics.avilaops.com/avila-tracker.js"
        data-site="G-AVILAOPS-01"
        data-debug="true">
</script>
```

No console você verá:
```
🔍 [Avila Analytics Debug]
✅ Initialized with ID: G-AVILAOPS-01
📊 Event tracked: page_view
📊 Event tracked: click (button.cta)
```

---

## 🛒 E-commerce Tracking

### Para Lojas Online

#### Visualização de Produto
```html
<script>
// Quando usuário visualiza um produto
avila.trackEcommerce('view_item', {
    items: [{
        item_id: 'SKU123',
        item_name: 'Produto X',
        price: 99.90,
        category: 'Eletrônicos',
        brand: 'Marca Y'
    }]
});
</script>
```

#### Adicionar ao Carrinho
```html
<button onclick="addToCart('SKU123')">
    Adicionar ao Carrinho
</button>

<script>
function addToCart(sku) {
    // Sua lógica de adicionar ao carrinho
    // ...
    
    // Track evento
    avila.trackEcommerce('add_to_cart', {
        items: [{
            item_id: sku,
            item_name: 'Produto X',
            price: 99.90,
            quantity: 1
        }]
    });
}
</script>
```

#### Compra Finalizada
```html
<script>
// Na página de "Pedido Confirmado"
avila.trackEcommerce('purchase', {
    transaction_id: 'TXN123456',
    value: 199.80,
    currency: 'BRL',
    tax: 10.00,
    shipping: 15.00,
    items: [{
        item_id: 'SKU123',
        item_name: 'Produto X',
        price: 99.90,
        quantity: 2
    }]
});
</script>
```

---

## 👤 User Identification

### Identificar Usuários Logados

```html
<script>
// Após login
avila.identify({
    user_id: 'user_12345',
    user_email_hash: 'abc123...', // hash do email
    user_properties: {
        plan: 'premium',
        signup_date: '2024-01-15'
    }
});
</script>
```

**⚠️ Importante:** Nunca envie PII (email, nome, CPF) em texto plano. Use hashes.

---

## 🔒 Privacy & GDPR

### Cookie Consent

Se você usa banner de cookies, integre assim:

```html
<script>
// Aguardar consent do usuário
document.addEventListener('cookie-consent-granted', function() {
    // Só então carregar analytics
    var script = document.createElement('script');
    script.src = 'https://analytics.avilaops.com/avila-tracker.js';
    script.setAttribute('data-site', 'G-AVILAOPS-01');
    document.body.appendChild(script);
});
</script>
```

### Opt-out Link

Permitir usuários optarem por não serem tracked:

```html
<a href="#" onclick="avila.optOut(); return false;">
    Não rastrear minhas visitas
</a>
```

---

## 📊 UTM Parameters

### Rastreamento de Campanhas

O tracker automaticamente captura UTM params:

```
https://avilaops.com/?utm_source=google&utm_medium=cpc&utm_campaign=lancamento
```

Será tracked como:
```javascript
{
    utm_source: 'google',
    utm_medium: 'cpc',
    utm_campaign: 'lancamento'
}
```

### Como Usar

**Google Ads:**
```
https://avilaops.com/?utm_source=google&utm_medium=cpc&utm_campaign=black_friday
```

**Facebook Ads:**
```
https://avilaops.com/?utm_source=facebook&utm_medium=social&utm_campaign=post_promo
```

**Email Marketing:**
```
https://avilaops.com/?utm_source=newsletter&utm_medium=email&utm_campaign=nov_2024
```

---

## 🐛 Troubleshooting

### Tracker não está funcionando

1. **Verificar se script carregou:**
   ```javascript
   // No console
   console.log(typeof avila); // Deve retornar 'object'
   ```

2. **Verificar erros no console:**
   - Abra DevTools (F12)
   - Vá para "Console"
   - Veja se há erros

3. **Verificar Network:**
   - Vá para tab "Network"
   - Recarregue a página
   - Procure por requisições para `analytics.avilaops.com`

### Events não aparecem no dashboard

1. **Aguarde 1-2 minutos** (pode haver delay)

2. **Verifique Measurement ID:**
   ```html
   <!-- Certifique-se de que está correto -->
   data-site="G-AVILAOPS-01"
   ```

3. **Modo debug:**
   ```html
   data-debug="true"
   ```

### CORS errors

Se vir erro de CORS, contate a equipe de infraestrutura para adicionar seu domínio na whitelist.

---

## 📱 Responsivo & Mobile

O tracker funciona automaticamente em:
- ✅ Desktop
- ✅ Tablets
- ✅ Smartphones
- ✅ PWAs
- ✅ WebView (apps)

Coleta automaticamente:
- Device type (desktop, mobile, tablet)
- Screen resolution
- Browser & OS
- Network speed

---

## ⚡ Performance

### Otimizações

O tracker é:
- ✅ **Leve**: <5KB minified + gzipped
- ✅ **Assíncrono**: Não bloqueia page load
- ✅ **Cached**: Cache agressivo (1 ano)
- ✅ **CDN**: Servido via CDN

### Async Loading

Para máxima performance:

```html
<script async src="https://analytics.avilaops.com/avila-tracker.js"
        data-site="G-AVILAOPS-01">
</script>
```

---

## 📚 Exemplos de Sites

### Landing Page Simples

```html
<!DOCTYPE html>
<html lang="pt-BR">
<head>
    <meta charset="UTF-8">
    <title>Minha Landing Page</title>
</head>
<body>
    <h1>Bem-vindo!</h1>
    
    <button id="cta">
        Solicitar Demo
    </button>
    
    <script src="https://analytics.avilaops.com/avila-tracker.js"
            data-site="G-LANDING-01">
    </script>
    
    <script>
    // Track CTA click
    document.getElementById('cta').addEventListener('click', function() {
        avila.track('cta_click', { location: 'hero' });
    });
    </script>
</body>
</html>
```

### Site Institucional Completo

```html
<!DOCTYPE html>
<html lang="pt-BR">
<head>
    <meta charset="UTF-8">
    <title>AvilaOps - DevOps Excellence</title>
</head>
<body>
    <!-- Header -->
    <header>
        <nav>
            <a href="/">Home</a>
            <a href="/sobre">Sobre</a>
            <a href="/servicos">Serviços</a>
            <a href="/contato">Contato</a>
        </nav>
    </header>
    
    <!-- Hero Section -->
    <section id="hero">
        <h1>Transforme sua Infraestrutura</h1>
        <button id="cta-hero">Fale Conosco</button>
    </section>
    
    <!-- Serviços -->
    <section id="services">
        <h2>Nossos Serviços</h2>
        <div class="service" data-service="kubernetes">
            <h3>Kubernetes</h3>
            <button class="learn-more">Saiba Mais</button>
        </div>
    </section>
    
    <!-- Footer -->
    <footer>
        <a href="https://wa.me/5511999999999" id="whatsapp">
            WhatsApp
        </a>
    </footer>
    
    <!-- Avila Analytics -->
    <script src="https://analytics.avilaops.com/avila-tracker.js"
            data-site="G-AVILAOPS-01">
    </script>
    
    <!-- Custom Tracking -->
    <script>
    // CTA Hero
    document.getElementById('cta-hero').addEventListener('click', function() {
        avila.track('cta_click', {
            location: 'hero',
            cta_text: 'Fale Conosco'
        });
    });
    
    // Learn More buttons
    document.querySelectorAll('.learn-more').forEach(function(btn) {
        btn.addEventListener('click', function() {
            var service = this.closest('.service').dataset.service;
            avila.track('service_interest', {
                service: service
            });
        });
    });
    
    // WhatsApp click
    document.getElementById('whatsapp').addEventListener('click', function() {
        avila.track('contact_method', {
            method: 'whatsapp',
            location: 'footer'
        });
    });
    </script>
</body>
</html>
```

---

## ✅ Checklist de Implementação

- [ ] Script adicionado antes de `</body>`
- [ ] Measurement ID correto
- [ ] Testado no console (avila object existe)
- [ ] Pageviews aparecendo no dashboard
- [ ] CTAs principais trackados
- [ ] Formulários trackados (se houver)
- [ ] Links externos trackados
- [ ] UTM parameters funcionando
- [ ] Mobile testado
- [ ] GDPR compliant (cookie consent)

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
