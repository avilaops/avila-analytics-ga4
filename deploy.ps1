#!/usr/bin/env pwsh
# Deploy script for Avila Analytics GA4

Write-Host "🚀 Avila Analytics GA4 - Deploy Script" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""

# Check if Docker is installed
if (-not (Get-Command docker -ErrorAction SilentlyContinue)) {
    Write-Host "❌ Docker is not installed. Please install Docker Desktop first." -ForegroundColor Red
    exit 1
}

# Check if docker-compose is available
if (-not (Get-Command docker-compose -ErrorAction SilentlyContinue)) {
    Write-Host "❌ docker-compose is not installed. Please install it first." -ForegroundColor Red
    exit 1
}

# Create .env file if it doesn't exist
if (-not (Test-Path ".env")) {
    Write-Host "📝 Creating .env file from .env.example..." -ForegroundColor Yellow
    Copy-Item ".env.example" ".env"
    Write-Host "✅ Please edit .env file with your configuration" -ForegroundColor Green
    Write-Host ""
}

# Create data and logs directories
Write-Host "📁 Creating data directories..." -ForegroundColor Yellow
New-Item -ItemType Directory -Force -Path ".\data" | Out-Null
New-Item -ItemType Directory -Force -Path ".\logs" | Out-Null
New-Item -ItemType Directory -Force -Path ".\config" | Out-Null

# Build the project
Write-Host "🔨 Building Rust project..." -ForegroundColor Yellow
cargo build --release
if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ Build failed!" -ForegroundColor Red
    exit 1
}
Write-Host "✅ Build completed successfully" -ForegroundColor Green
Write-Host ""

# Start services
Write-Host "🐳 Starting Docker services..." -ForegroundColor Yellow
docker-compose up -d postgres redis

# Wait for database to be ready
Write-Host "⏳ Waiting for database to be ready..." -ForegroundColor Yellow
Start-Sleep -Seconds 10

# Check database health
$dbReady = $false
$retries = 0
$maxRetries = 30

while (-not $dbReady -and $retries -lt $maxRetries) {
    $result = docker-compose exec -T postgres pg_isready -U postgres 2>&1
    if ($LASTEXITCODE -eq 0) {
        $dbReady = $true
    } else {
        $retries++
        Write-Host "." -NoNewline
        Start-Sleep -Seconds 2
    }
}

if (-not $dbReady) {
    Write-Host ""
    Write-Host "❌ Database failed to start in time!" -ForegroundColor Red
    exit 1
}

Write-Host ""
Write-Host "✅ Database is ready" -ForegroundColor Green
Write-Host ""

# Build and start analytics server
Write-Host "🚀 Starting Analytics server..." -ForegroundColor Yellow
docker-compose up -d analytics

# Wait for server to be ready
Write-Host "⏳ Waiting for server to be ready..." -ForegroundColor Yellow
Start-Sleep -Seconds 5

# Health check
$serverReady = $false
$retries = 0

while (-not $serverReady -and $retries -lt $maxRetries) {
    try {
        $response = Invoke-WebRequest -Uri "http://localhost:8080/health" -TimeoutSec 2 -ErrorAction SilentlyContinue
        if ($response.StatusCode -eq 200) {
            $serverReady = $true
        }
    } catch {
        $retries++
        Write-Host "." -NoNewline
        Start-Sleep -Seconds 2
    }
}

Write-Host ""

if ($serverReady) {
    Write-Host ""
    Write-Host "✅ Avila Analytics is running!" -ForegroundColor Green
    Write-Host ""
    Write-Host "📊 Dashboard: http://localhost:8080" -ForegroundColor Cyan
    Write-Host "🔌 API: http://localhost:8080/api/v1" -ForegroundColor Cyan
    Write-Host "💾 PostgreSQL: localhost:5432" -ForegroundColor Cyan
    Write-Host "🔴 Redis: localhost:6379" -ForegroundColor Cyan
    Write-Host ""
    Write-Host "📝 View logs: docker-compose logs -f analytics" -ForegroundColor Yellow
    Write-Host "🛑 Stop: docker-compose down" -ForegroundColor Yellow
    Write-Host ""
} else {
    Write-Host "❌ Server failed to start!" -ForegroundColor Red
    Write-Host "📝 Check logs: docker-compose logs analytics" -ForegroundColor Yellow
    exit 1
}
