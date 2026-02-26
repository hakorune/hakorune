#!/bin/bash
# 🚀 Nyash WASM Build Script

set -e  # Exit on error

echo "🐱 Building Nyash WebAssembly..."

# Check if wasm-pack is installed
if ! command -v wasm-pack &> /dev/null; then
    echo "❌ wasm-pack not found! Installing..."
    cargo install wasm-pack
fi

# Go to bridge crate
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
BRIDGE_DIR="${SCRIPT_DIR}/bridge"
OUT_DIR="${SCRIPT_DIR}/pkg"

# Build WASM package
echo "🔨 Building WASM package..."
cd "${BRIDGE_DIR}"
wasm-pack build --target web --out-dir "${OUT_DIR}"

# Return to wasm project directory
cd "${SCRIPT_DIR}"

echo "✅ Build complete!"
echo ""
echo "🌐 To test in browser:"
echo "1. python3 -m http.server 8000"
echo "2. Open: http://localhost:8000/nyash_playground.html"
echo ""
echo "📁 Generated files in pkg/:"
ls -la pkg/ 2>/dev/null || echo "   (run build first)"
