# 🐍 Guia de Integração - Python/Django/Flask

> **Para aplicações backend Python**

---

## 📋 Pré-requisitos

- ✅ Python 3.8+
- ✅ requests library
- ✅ Measurement ID fornecido pela equipe Analytics

---

## 🚀 Quick Start

### Instalação

```bash
pip install requests
# ou
pip install avila-analytics-python  # (quando publicado)
```

### Uso Básico

```python
import requests
import json

ANALYTICS_ENDPOINT = 'https://analytics.avilaops.com/api/v1/collect'
MEASUREMENT_ID = 'G-XXXXXXXXXX'

def track_event(event_name, params=None):
    """Track analytics event"""
    try:
        response = requests.post(
            ANALYTICS_ENDPOINT,
            json={
                'measurement_id': MEASUREMENT_ID,
                'event_name': event_name,
                'event_params': params or {},
                'timestamp': datetime.now().isoformat()
            },
            timeout=2
        )
        return response.status_code == 200
    except Exception as e:
        # Não bloquear aplicação por falha de analytics
        print(f"Analytics error: {e}")
        return False

# Uso
track_event('page_view', {
    'page_title': 'Home',
    'page_location': '/home'
})
```

---

## 🎯 Módulo Analytics Completo

### analytics.py

```python
"""
Avila Analytics - Python Client
"""
import requests
import json
from datetime import datetime
from typing import Dict, Any, Optional
from functools import wraps
import threading
import queue
import time

class AvilaAnalytics:
    """Cliente Python para Avila Analytics"""
    
    def __init__(
        self,
        measurement_id: str,
        endpoint: str = 'https://analytics.avilaops.com/api/v1/collect',
        batch_size: int = 10,
        flush_interval: float = 5.0,
        timeout: float = 2.0
    ):
        self.measurement_id = measurement_id
        self.endpoint = endpoint
        self.batch_size = batch_size
        self.flush_interval = flush_interval
        self.timeout = timeout
        
        # Batch queue
        self._queue = queue.Queue()
        self._batch = []
        self._last_flush = time.time()
        self._lock = threading.Lock()
        
        # Background worker
        self._worker = threading.Thread(target=self._worker_loop, daemon=True)
        self._worker.start()
    
    def track(self, event_name: str, params: Optional[Dict[str, Any]] = None):
        """Track event (async)"""
        event = {
            'measurement_id': self.measurement_id,
            'event_name': event_name,
            'event_params': params or {},
            'timestamp': datetime.now().isoformat()
        }
        
        self._queue.put(event)
    
    def track_sync(self, event_name: str, params: Optional[Dict[str, Any]] = None) -> bool:
        """Track event (sync)"""
        event = {
            'measurement_id': self.measurement_id,
            'event_name': event_name,
            'event_params': params or {},
            'timestamp': datetime.now().isoformat()
        }
        
        return self._send_event(event)
    
    def _send_event(self, event: Dict[str, Any]) -> bool:
        """Send single event"""
        try:
            response = requests.post(
                self.endpoint,
                json=event,
                timeout=self.timeout
            )
            return response.status_code == 200
        except Exception as e:
            print(f"Analytics error: {e}")
            return False
    
    def _send_batch(self, events: list) -> bool:
        """Send batch of events"""
        try:
            response = requests.post(
                f"{self.endpoint}/batch",
                json={'events': events},
                timeout=self.timeout
            )
            return response.status_code == 200
        except Exception as e:
            print(f"Analytics batch error: {e}")
            return False
    
    def _worker_loop(self):
        """Background worker for batch processing"""
        while True:
            try:
                # Get event from queue (with timeout)
                event = self._queue.get(timeout=1.0)
                
                with self._lock:
                    self._batch.append(event)
                    
                    # Flush if batch is full or interval elapsed
                    should_flush = (
                        len(self._batch) >= self.batch_size or
                        time.time() - self._last_flush >= self.flush_interval
                    )
                    
                    if should_flush:
                        self._flush()
                
            except queue.Empty:
                # Flush on timeout
                with self._lock:
                    if self._batch and time.time() - self._last_flush >= self.flush_interval:
                        self._flush()
    
    def _flush(self):
        """Flush batch"""
        if not self._batch:
            return
        
        events = self._batch[:]
        self._batch = []
        self._last_flush = time.time()
        
        # Send batch
        self._send_batch(events)
    
    def flush(self):
        """Force flush"""
        with self._lock:
            self._flush()


# Global singleton
_analytics = None

def init_analytics(measurement_id: str, **kwargs):
    """Initialize global analytics client"""
    global _analytics
    _analytics = AvilaAnalytics(measurement_id, **kwargs)

def track_event(event_name: str, params: Optional[Dict[str, Any]] = None):
    """Track event using global client"""
    if _analytics:
        _analytics.track(event_name, params)

def track_event_sync(event_name: str, params: Optional[Dict[str, Any]] = None) -> bool:
    """Track event synchronously using global client"""
    if _analytics:
        return _analytics.track_sync(event_name, params)
    return False
```

---

## 🌐 Django Integration

### settings.py

```python
# Analytics configuration
AVILA_ANALYTICS = {
    'MEASUREMENT_ID': 'G-DJANGO-01',
    'ENDPOINT': 'https://analytics.avilaops.com/api/v1/collect',
    'ENABLED': True,
    'BATCH_SIZE': 10,
}
```

### analytics/middleware.py

```python
from django.utils.deprecation import MiddlewareMixin
from django.conf import settings
import analytics

class AnalyticsMiddleware(MiddlewareMixin):
    """Track page views automatically"""
    
    def process_response(self, request, response):
        if settings.AVILA_ANALYTICS['ENABLED']:
            analytics.track_event('page_view', {
                'path': request.path,
                'method': request.method,
                'status_code': response.status_code,
                'user_agent': request.META.get('HTTP_USER_AGENT', ''),
            })
        
        return response
```

### analytics/decorators.py

```python
from functools import wraps
import analytics

def track_view(event_name=None):
    """Decorator to track view calls"""
    def decorator(view_func):
        @wraps(view_func)
        def wrapper(request, *args, **kwargs):
            # Track event
            analytics.track_event(
                event_name or f"view_{view_func.__name__}",
                {
                    'path': request.path,
                    'method': request.method,
                }
            )
            
            # Call original view
            return view_func(request, *args, **kwargs)
        
        return wrapper
    return decorator

# Uso
@track_view('home_page_view')
def home(request):
    return render(request, 'home.html')
```

### views.py

```python
from django.shortcuts import render
from django.http import JsonResponse
import analytics

def home(request):
    """Home page"""
    analytics.track_event('page_view', {
        'page_title': 'Home',
        'page_location': request.build_absolute_uri()
    })
    
    return render(request, 'home.html')

def api_endpoint(request):
    """API endpoint"""
    # Process request
    data = process_data(request.POST)
    
    # Track API call
    analytics.track_event('api_call', {
        'endpoint': '/api/data',
        'method': request.method,
        'status': 'success'
    })
    
    return JsonResponse({'status': 'ok', 'data': data})

def user_signup(request):
    """User signup"""
    if request.method == 'POST':
        user = create_user(request.POST)
        
        # Track signup
        analytics.track_event('user_signup', {
            'method': 'email',
            'user_id': user.id
        })
        
        return redirect('home')
    
    return render(request, 'signup.html')
```

### models.py

```python
from django.db import models
from django.db.models.signals import post_save
from django.dispatch import receiver
import analytics

class Order(models.Model):
    user = models.ForeignKey(User, on_delete=models.CASCADE)
    total = models.DecimalField(max_digits=10, decimal_places=2)
    status = models.CharField(max_length=20)
    created_at = models.DateTimeField(auto_now_add=True)

@receiver(post_save, sender=Order)
def track_order_created(sender, instance, created, **kwargs):
    """Track order creation"""
    if created:
        analytics.track_event('purchase', {
            'transaction_id': str(instance.id),
            'value': float(instance.total),
            'currency': 'BRL',
            'user_id': instance.user.id
        })
```

---

## 🚀 Flask Integration

### app.py

```python
from flask import Flask, request, jsonify
import analytics

app = Flask(__name__)

# Initialize analytics
analytics.init_analytics('G-FLASK-01')

@app.before_request
def track_request():
    """Track all requests"""
    analytics.track_event('page_view', {
        'path': request.path,
        'method': request.method,
    })

@app.route('/')
def home():
    return render_template('home.html')

@app.route('/api/data', methods=['POST'])
def api_data():
    data = request.get_json()
    
    # Process data
    result = process_data(data)
    
    # Track API call
    analytics.track_event('api_call', {
        'endpoint': '/api/data',
        'status': 'success'
    })
    
    return jsonify(result)

@app.route('/signup', methods=['POST'])
def signup():
    data = request.get_json()
    user = create_user(data)
    
    # Track signup
    analytics.track_event('user_signup', {
        'method': 'email',
        'user_id': user.id
    })
    
    return jsonify({'status': 'ok', 'user_id': user.id})

if __name__ == '__main__':
    app.run()
```

### Decorator Pattern

```python
from functools import wraps
from flask import request
import analytics

def track_endpoint(event_name=None):
    """Decorator to track endpoint calls"""
    def decorator(f):
        @wraps(f)
        def wrapper(*args, **kwargs):
            # Track event
            analytics.track_event(
                event_name or f"endpoint_{f.__name__}",
                {
                    'path': request.path,
                    'method': request.method,
                }
            )
            
            # Call original function
            return f(*args, **kwargs)
        
        return wrapper
    return decorator

# Uso
@app.route('/api/users')
@track_endpoint('get_users')
def get_users():
    return jsonify(User.query.all())
```

---

## 📊 Use Cases

### 1. Sistema de XMLs Fiscais

```python
import analytics

def process_xml(xml_file):
    """Process fiscal XML"""
    start_time = time.time()
    
    try:
        # Process XML
        result = parse_and_save_xml(xml_file)
        
        # Track success
        analytics.track_event('xml_processed', {
            'type': result.type,
            'status': 'success',
            'duration': time.time() - start_time,
            'file_size': xml_file.size
        })
        
        return result
        
    except Exception as e:
        # Track error
        analytics.track_event('xml_processing_error', {
            'type': xml_file.type,
            'error': str(e),
            'duration': time.time() - start_time
        })
        
        raise
```

### 2. Auth Service

```python
from flask import Flask, request, jsonify
from flask_jwt_extended import create_access_token
import analytics

app = Flask(__name__)

@app.route('/auth/login', methods=['POST'])
def login():
    """User login"""
    data = request.get_json()
    
    user = authenticate(data['email'], data['password'])
    
    if user:
        token = create_access_token(identity=user.id)
        
        # Track login
        analytics.track_event('user_login', {
            'method': 'email',
            'user_id': user.id,
            'success': True
        })
        
        return jsonify({'token': token})
    else:
        # Track failed login
        analytics.track_event('login_failed', {
            'method': 'email',
            'reason': 'invalid_credentials'
        })
        
        return jsonify({'error': 'Invalid credentials'}), 401

@app.route('/auth/logout', methods=['POST'])
def logout():
    """User logout"""
    user_id = get_current_user_id()
    
    # Track logout
    analytics.track_event('user_logout', {
        'user_id': user_id
    })
    
    return jsonify({'status': 'ok'})
```

### 3. Background Tasks (Celery)

```python
from celery import Celery
import analytics

app = Celery('tasks')

@app.task
def process_report(report_id):
    """Process report in background"""
    start_time = time.time()
    
    try:
        report = Report.objects.get(id=report_id)
        result = generate_report(report)
        
        # Track success
        analytics.track_event_sync('report_generated', {
            'report_id': report_id,
            'report_type': report.type,
            'duration': time.time() - start_time,
            'status': 'success'
        })
        
        return result
        
    except Exception as e:
        # Track error
        analytics.track_event_sync('report_generation_error', {
            'report_id': report_id,
            'error': str(e),
            'duration': time.time() - start_time
        })
        
        raise
```

---

## ⚡ Performance Tips

### 1. Async Tracking

```python
import threading

def track_async(event_name, params=None):
    """Track event asynchronously"""
    def _track():
        analytics.track_event(event_name, params)
    
    thread = threading.Thread(target=_track)
    thread.start()

# Uso
track_async('page_view', {'path': '/home'})
```

### 2. Batch Processing

```python
# Use o cliente com batching
analytics = AvilaAnalytics(
    measurement_id='G-XXXXXXXXXX',
    batch_size=50,  # Enviar a cada 50 eventos
    flush_interval=5.0  # ou a cada 5 segundos
)

# Events são batched automaticamente
for i in range(1000):
    analytics.track('test_event', {'index': i})

# Forçar flush no final
analytics.flush()
```

### 3. Context Manager

```python
from contextlib import contextmanager
import time

@contextmanager
def track_duration(event_name, extra_params=None):
    """Track event with duration"""
    start_time = time.time()
    
    try:
        yield
    finally:
        duration = time.time() - start_time
        params = {'duration': duration}
        if extra_params:
            params.update(extra_params)
        
        analytics.track_event(event_name, params)

# Uso
with track_duration('database_query', {'query': 'SELECT * FROM users'}):
    results = db.query('SELECT * FROM users')
```

---

## 🧪 Testing

### Mock for Tests

```python
from unittest.mock import Mock, patch
import analytics

def test_user_signup():
    """Test user signup tracks event"""
    
    # Mock analytics
    with patch('analytics.track_event') as mock_track:
        # Test code
        user = create_user({
            'email': 'test@example.com',
            'password': 'test123'
        })
        
        # Assert event was tracked
        mock_track.assert_called_once_with('user_signup', {
            'method': 'email',
            'user_id': user.id
        })
```

---

## ✅ Checklist de Implementação

- [ ] Módulo analytics.py criado
- [ ] Analytics inicializado na aplicação
- [ ] Middleware configurado (Django) ou before_request (Flask)
- [ ] Eventos principais tracked
- [ ] Decorators criados para tracking automático
- [ ] Testes escritos
- [ ] Error handling implementado
- [ ] Performance otimizada (async, batch)

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
