# Build frontend
Write-Host "🔨 Building Avila Analytics Frontend..." -ForegroundColor Green

# Build WASM Dashboard
Write-Host "📦 Building WebAssembly Dashboard..." -ForegroundColor Cyan
Set-Location frontend/wasm-dashboard
wasm-pack build --target web --out-dir ../../static/pkg
Set-Location ../..

# Copy tracker
Write-Host "⚡ Copying tracker..." -ForegroundColor Cyan
Copy-Item frontend/tracker/avila-tracker.js frontend/static/avila-tracker.js

Write-Host "✅ Frontend build complete!" -ForegroundColor Green
Write-Host ""
Write-Host "📂 Output files:" -ForegroundColor Yellow
Write-Host "   - frontend/static/pkg/          (WASM dashboard)"
Write-Host "   - frontend/static/avila-tracker.js"
Write-Host ""
Write-Host "🚀 To run the dashboard:" -ForegroundColor Yellow
Write-Host "   cd frontend/static; python -m http.server 8000"
