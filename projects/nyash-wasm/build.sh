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
PREBUILT_DIR="${SCRIPT_DIR}/prebuilt"
ROOT_DIR="$(cd "${SCRIPT_DIR}/../.." && pwd)"

# Build WASM package
echo "🔨 Building WASM package..."
cd "${BRIDGE_DIR}"
wasm-pack build --target web --out-dir "${OUT_DIR}"

# Build static prebuilt wasm demos for playground
echo "🧩 Building static prebuilt wasm demos..."
mkdir -p "${PREBUILT_DIR}"

compile_prebuilt() {
    local fixture_rel="$1"
    local out_name="$2"
    local fixture_abs="${ROOT_DIR}/${fixture_rel}"
    local out_base="${PREBUILT_DIR}/${out_name}"

    if [ ! -f "${fixture_abs}" ]; then
        echo "❌ fixture not found: ${fixture_abs}"
        exit 1
    fi

    if [ -x "${ROOT_DIR}/target/release/hakorune" ]; then
        if "${ROOT_DIR}/target/release/hakorune" --compile-wasm -o "${out_base}" "${fixture_abs}" >/dev/null 2>&1; then
            return
        fi
    fi
    if [ -x "${ROOT_DIR}/target/debug/hakorune" ]; then
        if "${ROOT_DIR}/target/debug/hakorune" --compile-wasm -o "${out_base}" "${fixture_abs}" >/dev/null 2>&1; then
            return
        fi
    fi

    (cd "${ROOT_DIR}" && cargo run --quiet --features wasm-backend --bin hakorune -- --compile-wasm -o "${out_base}" "${fixture_abs}" >/dev/null)
}

compile_prebuilt "apps/tests/phase29cc_wsm02d_demo_min.hako" "phase29cc_wsm02d_demo_min"
compile_prebuilt "apps/tests/phase29cc_wsm_g4_min3_webcanvas_fixture_min.hako" "phase29cc_wsm_g4_min3_webcanvas_fixture_min"
compile_prebuilt "apps/tests/phase29cc_wsm_g4_min4_canvas_advanced_fixture_min.hako" "phase29cc_wsm_g4_min4_canvas_advanced_fixture_min"

# Return to wasm project directory
cd "${SCRIPT_DIR}"

echo "✅ Build complete!"
echo ""
echo "🌐 To test in browser:"
echo "1. cd projects/nyash-wasm"
echo "2. python3 -m http.server 8000"
echo "3. Open: http://localhost:8000/nyash_playground.html"
echo ""
echo "📁 Generated files in pkg/:"
ls -la pkg/ 2>/dev/null || echo "   (run build first)"
echo ""
echo "📁 Generated prebuilt demos in prebuilt/:"
ls -la prebuilt/ 2>/dev/null || echo "   (run build first)"
