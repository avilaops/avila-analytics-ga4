# 📘 Guia de Integração - TypeScript/JavaScript

> **Para aplicações SaaS, React, Next.js, Node.js**

---

## 📋 Pré-requisitos

- ✅ Node.js 16+
- ✅ TypeScript 4.5+ (opcional)
- ✅ Measurement ID fornecido pela equipe Analytics

---

## 🚀 Quick Start

### Instalação

```bash
npm install @avila/analytics
# ou
yarn add @avila/analytics
# ou quando não publicado, use REST API diretamente
```

### Uso Básico

```typescript
import { AvilaAnalytics } from '@avila/analytics';

// Inicializar
const analytics = new AvilaAnalytics('G-XXXXXXXXXX');

// Track event
analytics.track('page_view', {
    page_title: 'Home',
    page_location: window.location.href
});
```

---

## 📦 SDK JavaScript/TypeScript

### analytics.ts

```typescript
/**
 * Avila Analytics - TypeScript/JavaScript Client
 */

export interface EventParams {
    [key: string]: string | number | boolean;
}

export interface AnalyticsConfig {
    measurementId: string;
    endpoint?: string;
    batchSize?: number;
    flushInterval?: number;
    debug?: boolean;
}

export class AvilaAnalytics {
    private config: Required<AnalyticsConfig>;
    private queue: any[] = [];
    private flushTimer: any = null;

    constructor(config: string | AnalyticsConfig) {
        if (typeof config === 'string') {
            this.config = {
                measurementId: config,
                endpoint: 'https://analytics.avilaops.com/api/v1/collect',
                batchSize: 10,
                flushInterval: 5000,
                debug: false
            };
        } else {
            this.config = {
                measurementId: config.measurementId,
                endpoint: config.endpoint || 'https://analytics.avilaops.com/api/v1/collect',
                batchSize: config.batchSize || 10,
                flushInterval: config.flushInterval || 5000,
                debug: config.debug || false
            };
        }

        // Start flush timer
        this.startFlushTimer();
    }

    /**
     * Track event
     */
    track(eventName: string, params?: EventParams): void {
        const event = {
            measurement_id: this.config.measurementId,
            event_name: eventName,
            event_params: params || {},
            timestamp: new Date().toISOString()
        };

        if (this.config.debug) {
            console.log('📊 [Avila Analytics] Event tracked:', event);
        }

        this.queue.push(event);

        // Flush if batch is full
        if (this.queue.length >= this.config.batchSize) {
            this.flush();
        }
    }

    /**
     * Track page view
     */
    trackPageView(params?: EventParams): void {
        this.track('page_view', {
            page_title: document.title,
            page_location: window.location.href,
            referrer: document.referrer,
            ...params
        });
    }

    /**
     * Track e-commerce event
     */
    trackEcommerce(eventName: string, params: any): void {
        this.track(eventName, params);
    }

    /**
     * Identify user
     */
    identify(userId: string, properties?: EventParams): void {
        this.track('user_identify', {
            user_id: userId,
            ...properties
        });
    }

    /**
     * Flush queue
     */
    async flush(): Promise<void> {
        if (this.queue.length === 0) return;

        const events = [...this.queue];
        this.queue = [];

        try {
            const response = await fetch(`${this.config.endpoint}/batch`, {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify({ events })
            });

            if (!response.ok && this.config.debug) {
                console.error('❌ [Avila Analytics] Failed to send events:', response.statusText);
            }
        } catch (error) {
            if (this.config.debug) {
                console.error('❌ [Avila Analytics] Error sending events:', error);
            }
        }
    }

    /**
     * Start flush timer
     */
    private startFlushTimer(): void {
        this.flushTimer = setInterval(() => {
            this.flush();
        }, this.config.flushInterval);
    }

    /**
     * Stop tracking and flush
     */
    destroy(): void {
        if (this.flushTimer) {
            clearInterval(this.flushTimer);
        }
        this.flush();
    }
}

// Global instance
let globalAnalytics: AvilaAnalytics | null = null;

/**
 * Initialize global analytics
 */
export function initAnalytics(config: string | AnalyticsConfig): AvilaAnalytics {
    globalAnalytics = new AvilaAnalytics(config);
    return globalAnalytics;
}

/**
 * Get global analytics instance
 */
export function getAnalytics(): AvilaAnalytics {
    if (!globalAnalytics) {
        throw new Error('Analytics not initialized. Call initAnalytics() first.');
    }
    return globalAnalytics;
}

/**
 * Track event using global instance
 */
export function track(eventName: string, params?: EventParams): void {
    getAnalytics().track(eventName, params);
}

/**
 * Track page view using global instance
 */
export function trackPageView(params?: EventParams): void {
    getAnalytics().trackPageView(params);
}
```

---

## ⚛️ React Integration

### hooks/useAnalytics.ts

```typescript
import { useEffect } from 'react';
import { AvilaAnalytics } from '../lib/analytics';

/**
 * Analytics hook
 */
export function useAnalytics(measurementId: string) {
    useEffect(() => {
        const analytics = new AvilaAnalytics(measurementId);
        
        // Track initial page view
        analytics.trackPageView();
        
        // Cleanup on unmount
        return () => {
            analytics.destroy();
        };
    }, [measurementId]);
}

/**
 * Track event hook
 */
export function useTrackEvent() {
    const analytics = getAnalytics();
    
    return (eventName: string, params?: EventParams) => {
        analytics.track(eventName, params);
    };
}
```

### App.tsx

```typescript
import React from 'react';
import { useAnalytics } from './hooks/useAnalytics';

function App() {
    // Initialize analytics
    useAnalytics('G-XXXXXXXXXX');
    
    return (
        <div className="App">
            {/* Your app */}
        </div>
    );
}

export default App;
```

### Componente com Tracking

```typescript
import React from 'react';
import { useTrackEvent } from './hooks/useAnalytics';

function CallToAction() {
    const trackEvent = useTrackEvent();
    
    const handleClick = () => {
        trackEvent('cta_click', {
            cta_id: 'hero_button',
            cta_text: 'Get Started'
        });
        
        // Your CTA logic
    };
    
    return (
        <button onClick={handleClick}>
            Get Started
        </button>
    );
}
```

### React Router Integration

```typescript
import { useEffect } from 'react';
import { useLocation } from 'react-router-dom';
import { getAnalytics } from '../lib/analytics';

function AnalyticsTracker() {
    const location = useLocation();
    const analytics = getAnalytics();
    
    useEffect(() => {
        // Track page view on route change
        analytics.trackPageView({
            page_path: location.pathname
        });
    }, [location, analytics]);
    
    return null;
}

// In App.tsx
function App() {
    return (
        <Router>
            <AnalyticsTracker />
            {/* Routes */}
        </Router>
    );
}
```

---

## 🚀 Next.js Integration

### lib/analytics.ts

```typescript
import { AvilaAnalytics } from './avila-analytics';

export const analytics = new AvilaAnalytics({
    measurementId: process.env.NEXT_PUBLIC_ANALYTICS_ID!,
    debug: process.env.NODE_ENV === 'development'
});
```

### pages/_app.tsx

```typescript
import type { AppProps } from 'next/app';
import { useEffect } from 'react';
import { useRouter } from 'next/router';
import { analytics } from '../lib/analytics';

function MyApp({ Component, pageProps }: AppProps) {
    const router = useRouter();
    
    useEffect(() => {
        // Track page views on route change
        const handleRouteChange = (url: string) => {
            analytics.trackPageView({ page_path: url });
        };
        
        router.events.on('routeChangeComplete', handleRouteChange);
        
        return () => {
            router.events.off('routeChangeComplete', handleRouteChange);
        };
    }, [router.events]);
    
    return <Component {...pageProps} />;
}

export default MyApp;
```

### API Route Tracking

```typescript
// pages/api/users.ts
import type { NextApiRequest, NextApiResponse } from 'next';
import { analytics } from '../../lib/analytics';

export default async function handler(
    req: NextApiRequest,
    res: NextApiResponse
) {
    // Track API call
    analytics.track('api_call', {
        endpoint: '/api/users',
        method: req.method!,
    });
    
    // Your API logic
    const users = await getUsers();
    
    res.status(200).json(users);
}
```

---

## 🟢 Node.js Backend Integration

### server.ts

```typescript
import express from 'express';
import { AvilaAnalytics } from './analytics';

const app = express();
const analytics = new AvilaAnalytics('G-BACKEND-01');

// Middleware to track all requests
app.use((req, res, next) => {
    analytics.track('api_request', {
        path: req.path,
        method: req.method,
        user_agent: req.get('user-agent')
    });
    
    next();
});

// Track specific endpoints
app.post('/api/users', async (req, res) => {
    const user = await createUser(req.body);
    
    // Track user creation
    analytics.track('user_created', {
        user_id: user.id,
        method: 'api'
    });
    
    res.json(user);
});

app.listen(3000, () => {
    console.log('Server running on port 3000');
});
```

### Express Middleware

```typescript
import { Request, Response, NextFunction } from 'express';
import { AvilaAnalytics } from './analytics';

export function analyticsMiddleware(analytics: AvilaAnalytics) {
    return (req: Request, res: Response, next: NextFunction) => {
        const start = Date.now();
        
        // Track request on response finish
        res.on('finish', () => {
            const duration = Date.now() - start;
            
            analytics.track('http_request', {
                method: req.method,
                path: req.path,
                status_code: res.statusCode,
                duration_ms: duration
            });
        });
        
        next();
    };
}

// Usage
app.use(analyticsMiddleware(analytics));
```

---

## 📊 Use Cases

### 1. SaaS Application (AvilaInc)

```typescript
// Track feature usage
function exportPDF() {
    analytics.track('feature_used', {
        feature: 'export_pdf',
        user_plan: user.plan,
        document_type: 'report'
    });
    
    // Your export logic
}

// Track errors
try {
    await processData();
} catch (error) {
    analytics.track('error_occurred', {
        error_type: error.name,
        error_message: error.message,
        context: 'data_processing'
    });
    
    throw error;
}
```

### 2. E-commerce (ArkanaStore)

```typescript
// Product view
function viewProduct(product: Product) {
    analytics.trackEcommerce('view_item', {
        items: [{
            item_id: product.sku,
            item_name: product.name,
            price: product.price,
            category: product.category
        }]
    });
}

// Add to cart
function addToCart(product: Product, quantity: number) {
    analytics.trackEcommerce('add_to_cart', {
        items: [{
            item_id: product.sku,
            item_name: product.name,
            price: product.price,
            quantity: quantity
        }],
        value: product.price * quantity,
        currency: 'BRL'
    });
    
    // Your cart logic
}

// Purchase
function completePurchase(order: Order) {
    analytics.trackEcommerce('purchase', {
        transaction_id: order.id,
        value: order.total,
        currency: 'BRL',
        tax: order.tax,
        shipping: order.shipping,
        items: order.items.map(item => ({
            item_id: item.sku,
            item_name: item.name,
            price: item.price,
            quantity: item.quantity
        }))
    });
}
```

### 3. Dashboard (Pulse)

```typescript
// Track dashboard interactions
function onWidgetClick(widget: Widget) {
    analytics.track('widget_interaction', {
        widget_id: widget.id,
        widget_type: widget.type,
        action: 'click'
    });
}

// Track filter changes
function onFilterChange(filters: Filters) {
    analytics.track('filter_applied', {
        filter_type: filters.type,
        filter_value: filters.value
    });
}

// Track export
function exportData(format: string) {
    analytics.track('data_export', {
        format: format,
        rows: data.length
    });
}
```

---

## 🧪 Testing

### Mock for Jest

```typescript
// __mocks__/analytics.ts
export class AvilaAnalytics {
    track = jest.fn();
    trackPageView = jest.fn();
    trackEcommerce = jest.fn();
    identify = jest.fn();
    flush = jest.fn();
    destroy = jest.fn();
}

// test file
import { AvilaAnalytics } from '../lib/analytics';

jest.mock('../lib/analytics');

describe('Component', () => {
    it('tracks event on click', () => {
        const analytics = new AvilaAnalytics('test-id');
        
        // Test code
        component.handleClick();
        
        expect(analytics.track).toHaveBeenCalledWith('button_click', {
            button_id: 'test-button'
        });
    });
});
```

---

## ⚡ Performance Tips

### 1. Code Splitting

```typescript
// Lazy load analytics
const analytics = await import('./lib/analytics');
analytics.track('event', {});
```

### 2. Debounce High-Frequency Events

```typescript
import { debounce } from 'lodash';

const trackScroll = debounce(() => {
    analytics.track('scroll', {
        scroll_depth: getScrollDepth()
    });
}, 1000);

window.addEventListener('scroll', trackScroll);
```

### 3. Batch Processing

```typescript
const analytics = new AvilaAnalytics({
    measurementId: 'G-XXXXXXXXXX',
    batchSize: 50,  // Send every 50 events
    flushInterval: 5000  // or every 5 seconds
});
```

---

## 🔒 Privacy & GDPR

### Cookie Consent

```typescript
// Wait for consent
document.addEventListener('cookie-consent-granted', () => {
    const analytics = initAnalytics('G-XXXXXXXXXX');
    analytics.trackPageView();
});
```

### Opt-out

```typescript
// Allow users to opt out
function optOut() {
    localStorage.setItem('analytics-opt-out', 'true');
    analytics.destroy();
}

// Check opt-out status
if (!localStorage.getItem('analytics-opt-out')) {
    initAnalytics('G-XXXXXXXXXX');
}
```

---

## ✅ Checklist de Implementação

- [ ] SDK instalado ou analytics.ts criado
- [ ] Analytics inicializado na aplicação
- [ ] Page views tracked automaticamente
- [ ] Eventos principais tracked
- [ ] E-commerce events (se aplicável)
- [ ] Error tracking implementado
- [ ] Testes escritos
- [ ] GDPR compliant (cookie consent)
- [ ] Performance otimizada (batch, debounce)

---

## 📚 Exemplos Completos

Ver repositório:
- `examples/react-app/`
- `examples/nextjs-app/`
- `examples/nodejs-api/`

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
