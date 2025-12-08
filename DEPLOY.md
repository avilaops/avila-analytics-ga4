# 🚀 Deployment Guide - Avila Analytics GA4

## 📋 Pré-requisitos

- Docker Desktop 20.10+ ([Download](https://www.docker.com/products/docker-desktop))
- Docker Compose 2.0+
- Rust 1.75+ (apenas para build local)
- 4GB RAM mínimo
- 10GB espaço em disco

## 🎯 Quick Start (Docker)

### 1. Clone e Configure

```bash
cd d:\arxis\avila-analytics-ga4

# Copie o arquivo de exemplo
cp .env.example .env

# Edite as configurações (se necessário)
notepad .env
```

### 2. Deploy Completo

**Windows (PowerShell):**
```powershell
.\deploy.ps1
```

**Linux/Mac:**
```bash
chmod +x deploy.sh
./deploy.sh
```

O script irá:
- ✅ Verificar dependências
- ✅ Criar diretórios necessários
- ✅ Build do projeto Rust
- ✅ Iniciar PostgreSQL + Redis
- ✅ Iniciar servidor Analytics
- ✅ Verificar health checks

### 3. Acesse a Aplicação

- 📊 **Dashboard**: http://localhost:8080
- 🔌 **API**: http://localhost:8080/api/v1
- 💾 **PostgreSQL**: localhost:5432
- 🔴 **Redis**: localhost:6379

## 🐳 Docker Compose

### Comandos Úteis

```bash
# Iniciar todos os serviços
docker-compose up -d

# Iniciar apenas database
docker-compose up -d postgres redis

# Ver logs
docker-compose logs -f analytics

# Parar todos os serviços
docker-compose down

# Parar e remover volumes (limpa dados)
docker-compose down -v

# Rebuild da imagem
docker-compose build --no-cache analytics
```

### Perfis Disponíveis

```bash
# Com monitoring (Prometheus + Grafana)
docker-compose --profile monitoring up -d

# Apenas serviços essenciais (default)
docker-compose up -d
```

## 🔧 Build Manual

### Desenvolvimento Local

```bash
# Build debug
cargo build

# Build release
cargo build --release

# Rodar testes
cargo test

# Rodar servidor
cargo run --bin avila-analytics

# Rodar CLI
cargo run --bin avila-analytics-cli -- --help
```

### Frontend (WebAssembly)

```bash
# Install wasm-pack
cargo install wasm-pack

# Build WASM dashboard
cd frontend/wasm-dashboard
wasm-pack build --target web --out-dir ../static/pkg
```

## ⚙️ Configuração

### Variáveis de Ambiente

Todas as configurações podem ser sobrescritas via variáveis de ambiente com prefixo `AVILA_ANALYTICS_`:

```bash
# Servidor
AVILA_ANALYTICS_SERVER__HOST=0.0.0.0
AVILA_ANALYTICS_SERVER__PORT=8080
AVILA_ANALYTICS_SERVER__WORKERS=4

# Database
AVILA_ANALYTICS_DATABASE__URL=postgres://user:pass@host:5432/db
AVILA_ANALYTICS_DATABASE__MAX_CONNECTIONS=100

# Redis
AVILA_ANALYTICS_REDIS__URL=redis://host:6379

# Privacy
AVILA_ANALYTICS_PRIVACY__ANONYMIZE_IP=true
AVILA_ANALYTICS_PRIVACY__RESPECT_DNT=true
AVILA_ANALYTICS_PRIVACY__DATA_RETENTION_DAYS=365

# Storage
AVILA_ANALYTICS_STORAGE__BATCH_SIZE=1000
AVILA_ANALYTICS_STORAGE__FLUSH_INTERVAL_SECS=5
```

### Arquivo config.toml

Alternativamente, use o arquivo `config.toml`:

```toml
[server]
host = "0.0.0.0"
port = 8080

[database]
url = "postgres://postgres:postgres@localhost:5432/analytics"

[privacy]
anonymize_ip = true
respect_dnt = true
```

## 🔐 Segurança

### Produção

1. **Altere senhas padrão** no `.env`:
   ```bash
   POSTGRES_PASSWORD=sua_senha_forte_aqui
   ```

2. **Configure CORS** específico:
   ```bash
   AVILA_ANALYTICS_SERVER__CORS_ORIGINS=["https://seusite.com"]
   ```

3. **Enable HTTPS** (use nginx/traefik como proxy reverso)

4. **Firewall**: Exponha apenas porta 8080 (ou use proxy)

### Backup

```bash
# Backup PostgreSQL
docker-compose exec postgres pg_dump -U postgres analytics > backup.sql

# Restore
docker-compose exec -T postgres psql -U postgres analytics < backup.sql
```

## 📊 Monitoring

### Logs

```bash
# Logs do analytics
docker-compose logs -f analytics

# Logs de todos os serviços
docker-compose logs -f

# Últimas 100 linhas
docker-compose logs --tail=100 analytics
```

### Prometheus + Grafana

```bash
# Iniciar com monitoring
docker-compose --profile monitoring up -d

# Acessar
# Prometheus: http://localhost:9090
# Grafana: http://localhost:3000 (admin/admin)
```

### Health Check

```bash
curl http://localhost:8080/health
# Resposta: OK

# Métricas
curl http://localhost:8080/api/v1/metrics
```

## 🚀 Deploy em Produção

### Cloud Providers

#### AWS (ECS/Fargate)

1. Build e push da imagem:
```bash
docker build -t avila-analytics .
docker tag avila-analytics:latest <account>.dkr.ecr.<region>.amazonaws.com/avila-analytics:latest
docker push <account>.dkr.ecr.<region>.amazonaws.com/avila-analytics:latest
```

2. Configure RDS PostgreSQL e ElastiCache Redis

3. Deploy via ECS Task Definition

#### Google Cloud (Cloud Run)

```bash
# Build
gcloud builds submit --tag gcr.io/<project>/avila-analytics

# Deploy
gcloud run deploy avila-analytics \
  --image gcr.io/<project>/avila-analytics \
  --platform managed \
  --region us-central1 \
  --set-env-vars AVILA_ANALYTICS_DATABASE__URL=$DB_URL
```

#### Azure (Container Apps)

```bash
az containerapp create \
  --name avila-analytics \
  --resource-group myResourceGroup \
  --image avila-analytics:latest \
  --target-port 8080 \
  --env-vars \
    AVILA_ANALYTICS_DATABASE__URL=$DB_URL
```

### VPS/Dedicated Server

```bash
# Install Docker
curl -fsSL https://get.docker.com -o get-docker.sh
sh get-docker.sh

# Clone e deploy
git clone https://github.com/arxis/avila-analytics-ga4
cd avila-analytics-ga4
./deploy.sh
```

## 🐛 Troubleshooting

### Porta já em uso

```bash
# Mude a porta no .env
ANALYTICS_PORT=8081
docker-compose up -d
```

### Database não conecta

```bash
# Verifique se PostgreSQL está rodando
docker-compose ps

# Logs do PostgreSQL
docker-compose logs postgres

# Teste conexão
docker-compose exec postgres psql -U postgres analytics
```

### Build falha

```bash
# Limpe cache do Cargo
cargo clean
cargo build --release

# Ou rebuild completo no Docker
docker-compose build --no-cache
```

## 📚 Mais Informações

- [README.md](./README.md) - Visão geral do projeto
- [QUICKSTART.md](./QUICKSTART.md) - Guia rápido de uso
- [docs/ARCHITECTURE.md](./docs/ARCHITECTURE.md) - Arquitetura detalhada

## 🆘 Suporte

- Issues: https://github.com/arxis/avila-analytics-ga4/issues
- Docs: https://docs.avila-analytics.com
- Discord: https://discord.gg/avila
