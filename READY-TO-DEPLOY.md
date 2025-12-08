# 🚀 Avila Analytics GA4 - Ready for Deploy!

## ✅ Status: PRONTO PARA PRODUÇÃO

Todos os arquivos de deploy foram criados e o projeto compila com sucesso!

## 📦 Arquivos Criados

### Docker & Deploy
- ✅ `Dockerfile` - Multi-stage build (frontend WASM + backend Rust)
- ✅ `docker-compose.yml` - Orquestração completa (PostgreSQL + Redis + Analytics + Monitoring)
- ✅ `.env` - Variáveis de ambiente
- ✅ `.env.example` - Template de configuração
- ✅ `config.toml` - Configuração alternativa

### Scripts
- ✅ `deploy.ps1` - Script de deploy para Windows (PowerShell)
- ✅ `deploy.sh` - Script de deploy para Linux/Mac (Bash)
- ✅ `scripts/init-db.sql` - Schema do banco de dados

### Documentação
- ✅ `DEPLOY.md` - Guia completo de deployment
- ✅ `.gitignore` - Arquivos a ignorar no Git

## 🚀 Como Fazer o Deploy

### Opção 1: Deploy Automático (Recomendado)

**Windows:**
```powershell
cd d:\arxis\avila-analytics-ga4
.\deploy.ps1
```

**Linux/Mac:**
```bash
cd /path/to/avila-analytics-ga4
chmod +x deploy.sh
./deploy.sh
```

### Opção 2: Deploy Manual

```bash
# 1. Configurar variáveis de ambiente
cp .env.example .env
# Edite o .env com suas configurações

# 2. Iniciar serviços
docker-compose up -d postgres redis

# 3. Aguardar database estar pronto
sleep 10

# 4. Build e iniciar analytics
cargo build --release
docker-compose up -d analytics
```

### Opção 3: Docker Compose Completo

```bash
# Tudo de uma vez
docker-compose up -d

# Com monitoring (Prometheus + Grafana)
docker-compose --profile monitoring up -d
```

## 🌐 Endpoints Disponíveis

Após o deploy, você terá acesso a:

| Serviço | URL | Descrição |
|---------|-----|-----------|
| 📊 Dashboard | http://localhost:8080 | Interface web principal |
| 🔌 API | http://localhost:8080/api/v1 | REST API |
| 💚 Health Check | http://localhost:8080/health | Status do servidor |
| 📈 Metrics | http://localhost:8080/api/v1/metrics | Métricas do sistema |
| 💾 PostgreSQL | localhost:5432 | Database |
| 🔴 Redis | localhost:6379 | Cache |
| 📊 Prometheus | http://localhost:9090 | Monitoring (opcional) |
| 📈 Grafana | http://localhost:3000 | Dashboards (opcional) |

## 🔧 Configuração

### Variáveis de Ambiente Principais

```bash
# Servidor
ANALYTICS_PORT=8080
WORKERS=4

# Database
POSTGRES_DB=analytics
POSTGRES_USER=postgres
POSTGRES_PASSWORD=postgres  # ⚠️ MUDE EM PRODUÇÃO!
POSTGRES_PORT=5432

# Redis
REDIS_PORT=6379

# Logging
RUST_LOG=info  # debug, info, warn, error
```

### Configuração Avançada

Todas as configurações podem ser sobrescritas via variáveis de ambiente com prefixo `AVILA_ANALYTICS_`:

```bash
AVILA_ANALYTICS_SERVER__HOST=0.0.0.0
AVILA_ANALYTICS_SERVER__PORT=8080
AVILA_ANALYTICS_DATABASE__URL=postgres://user:pass@host:5432/db
AVILA_ANALYTICS_REDIS__URL=redis://host:6379
AVILA_ANALYTICS_PRIVACY__ANONYMIZE_IP=true
AVILA_ANALYTICS_STORAGE__BATCH_SIZE=1000
```

## 📊 Verificação do Deploy

### 1. Health Check

```bash
curl http://localhost:8080/health
# Resposta esperada: OK
```

### 2. Logs

```bash
# Ver logs do analytics
docker-compose logs -f analytics

# Ver logs de todos os serviços
docker-compose logs -f

# Últimas 100 linhas
docker-compose logs --tail=100 analytics
```

### 3. Métricas

```bash
curl http://localhost:8080/api/v1/metrics
# Retorna JSON com métricas do sistema
```

## 🧪 Testar o Sistema

### 1. Teste com curl

```bash
# Enviar evento de pageview
curl -X POST http://localhost:8080/api/v1/collect \
  -H "Content-Type: application/json" \
  -d '{
    "measurement_id": "G-TEST123",
    "client_id": "test-client-123",
    "events": [{
      "name": "page_view",
      "timestamp": "2025-12-08T12:00:00Z",
      "params": {
        "page_title": "Home",
        "page_location": "https://example.com"
      }
    }]
  }'
```

### 2. Abrir Demo no Navegador

```bash
# Windows
start frontend/static/demo.html

# Linux/Mac
open frontend/static/demo.html
```

## 🔐 Segurança em Produção

### ⚠️ IMPORTANTE: Antes de ir para produção

1. **Mude as senhas padrão**:
   ```bash
   POSTGRES_PASSWORD=SuaSenhaForteAqui123!
   ```

2. **Configure CORS específico**:
   ```bash
   AVILA_ANALYTICS_SERVER__CORS_ORIGINS=["https://seusite.com"]
   ```

3. **Use HTTPS**: Configure nginx/traefik como proxy reverso

4. **Firewall**: Exponha apenas a porta necessária (8080)

5. **Backup**: Configure backup automático do PostgreSQL

## 🛠️ Comandos Úteis

```bash
# Parar todos os serviços
docker-compose down

# Parar e remover volumes (limpa dados)
docker-compose down -v

# Rebuild da imagem
docker-compose build --no-cache analytics

# Reiniciar apenas o analytics
docker-compose restart analytics

# Ver status
docker-compose ps

# Backup do database
docker-compose exec postgres pg_dump -U postgres analytics > backup.sql

# Restore
docker-compose exec -T postgres psql -U postgres analytics < backup.sql
```

## 📚 Documentação Completa

- [README.md](./README.md) - Visão geral do projeto
- [QUICKSTART.md](./QUICKSTART.md) - Guia rápido
- [DEPLOY.md](./DEPLOY.md) - Guia completo de deploy
- [docs/ARCHITECTURE.md](./docs/ARCHITECTURE.md) - Arquitetura

## 🚢 Deploy em Cloud

### AWS

```bash
# Build e push
docker build -t avila-analytics .
docker tag avila-analytics:latest <account>.dkr.ecr.<region>.amazonaws.com/avila-analytics:latest
docker push <account>.dkr.ecr.<region>.amazonaws.com/avila-analytics:latest

# Use RDS para PostgreSQL e ElastiCache para Redis
```

### Google Cloud

```bash
# Build e deploy
gcloud builds submit --tag gcr.io/<project>/avila-analytics
gcloud run deploy avila-analytics --image gcr.io/<project>/avila-analytics
```

### Azure

```bash
az containerapp create \
  --name avila-analytics \
  --resource-group myResourceGroup \
  --image avila-analytics:latest
```

## ✅ Checklist de Deploy

- [ ] Docker e Docker Compose instalados
- [ ] Arquivo `.env` configurado
- [ ] Senhas alteradas (produção)
- [ ] CORS configurado (produção)
- [ ] Portas disponíveis (8080, 5432, 6379)
- [ ] Mínimo 4GB RAM disponível
- [ ] 10GB espaço em disco
- [ ] Health check funcionando
- [ ] Logs verificados
- [ ] Backup configurado (produção)

## 🎯 Próximos Passos

1. **Execute o deploy**: `.\deploy.ps1` ou `./deploy.sh`
2. **Verifique o health check**: `curl http://localhost:8080/health`
3. **Abra o dashboard**: http://localhost:8080
4. **Teste enviar eventos**: Use o demo.html ou curl
5. **Configure monitoring**: Ative Prometheus/Grafana se necessário
6. **Leia a documentação**: [DEPLOY.md](./DEPLOY.md) para detalhes

## 🆘 Troubleshooting

### Porta em uso
```bash
# Mude no .env
ANALYTICS_PORT=8081
```

### Database não conecta
```bash
docker-compose logs postgres
docker-compose restart postgres
```

### Build falha
```bash
cargo clean
cargo build --release
```

---

**🎉 Seu sistema de analytics está pronto para deploy!**

Execute `.\deploy.ps1` (Windows) ou `./deploy.sh` (Linux/Mac) para começar.
