#!/usr/bin/env bash
set -e

echo "🔨 Building Avila Analytics Frontend..."

# Build WASM Dashboard
echo "📦 Building WebAssembly Dashboard..."
cd frontend/wasm-dashboard
wasm-pack build --target web --out-dir ../../static/pkg
cd ../..

# Minify tracker
echo "⚡ Minifying tracker..."
if command -v terser &> /dev/null; then
    terser frontend/tracker/avila-tracker.js \
        --compress \
        --mangle \
        -o frontend/static/avila-tracker.min.js
    echo "   ✓ Minified tracker created"
else
    echo "   ⚠ terser not found, copying without minification"
    cp frontend/tracker/avila-tracker.js frontend/static/avila-tracker.js
fi

echo "✅ Frontend build complete!"
echo ""
echo "📂 Output files:"
echo "   - frontend/static/pkg/          (WASM dashboard)"
echo "   - frontend/static/avila-tracker.min.js"
echo ""
echo "🚀 To run the dashboard:"
echo "   cd frontend/static && python3 -m http.server 8000"
