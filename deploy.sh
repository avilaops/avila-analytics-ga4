#!/bin/bash
# Deploy script for Avila Analytics GA4

echo "🚀 Avila Analytics GA4 - Deploy Script"
echo "========================================"
echo ""

# Check if Docker is installed
if ! command -v docker &> /dev/null; then
    echo "❌ Docker is not installed. Please install Docker first."
    exit 1
fi

# Check if docker-compose is available
if ! command -v docker-compose &> /dev/null; then
    echo "❌ docker-compose is not installed. Please install it first."
    exit 1
fi

# Create .env file if it doesn't exist
if [ ! -f .env ]; then
    echo "📝 Creating .env file from .env.example..."
    cp .env.example .env
    echo "✅ Please edit .env file with your configuration"
    echo ""
fi

# Create data and logs directories
echo "📁 Creating data directories..."
mkdir -p data logs config

# Build the project
echo "🔨 Building Rust project..."
cargo build --release
if [ $? -ne 0 ]; then
    echo "❌ Build failed!"
    exit 1
fi
echo "✅ Build completed successfully"
echo ""

# Start services
echo "🐳 Starting Docker services..."
docker-compose up -d postgres redis

# Wait for database to be ready
echo "⏳ Waiting for database to be ready..."
sleep 10

# Check database health
retries=0
max_retries=30
db_ready=false

while [ $retries -lt $max_retries ] && [ "$db_ready" = false ]; do
    if docker-compose exec -T postgres pg_isready -U postgres > /dev/null 2>&1; then
        db_ready=true
    else
        retries=$((retries + 1))
        echo -n "."
        sleep 2
    fi
done

echo ""
if [ "$db_ready" = false ]; then
    echo "❌ Database failed to start in time!"
    exit 1
fi

echo "✅ Database is ready"
echo ""

# Build and start analytics server
echo "🚀 Starting Analytics server..."
docker-compose up -d analytics

# Wait for server to be ready
echo "⏳ Waiting for server to be ready..."
sleep 5

# Health check
retries=0
server_ready=false

while [ $retries -lt $max_retries ] && [ "$server_ready" = false ]; do
    if curl -f http://localhost:8080/health > /dev/null 2>&1; then
        server_ready=true
    else
        retries=$((retries + 1))
        echo -n "."
        sleep 2
    fi
done

echo ""

if [ "$server_ready" = true ]; then
    echo ""
    echo "✅ Avila Analytics is running!"
    echo ""
    echo "📊 Dashboard: http://localhost:8080"
    echo "🔌 API: http://localhost:8080/api/v1"
    echo "💾 PostgreSQL: localhost:5432"
    echo "🔴 Redis: localhost:6379"
    echo ""
    echo "📝 View logs: docker-compose logs -f analytics"
    echo "🛑 Stop: docker-compose down"
    echo ""
else
    echo "❌ Server failed to start!"
    echo "📝 Check logs: docker-compose logs analytics"
    exit 1
fi
