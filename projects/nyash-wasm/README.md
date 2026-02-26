# 🌐 Nyash WebAssembly Project

Status: `WSM-G2-min1` まで復旧済み。`projects/nyash-wasm/bridge` を wasm build のSSOTとし、`nyash_playground.html` の ConsoleBox 5メソッド最小run-loopを固定している。次は `WSM-G2-min2`（headless automation smoke）。

## 🚀 Quick Start (experimental)

```bash
# Install wasm-pack (if not already installed)
cargo install wasm-pack

# Build WASM module (bridge crate)
bash projects/nyash-wasm/build.sh

# Start local server (example)
cd projects/nyash-wasm
python3 -m http.server 8000

# Open browser
# Navigate to: http://localhost:8000/nyash_playground.html
```

## 🎯 Features (G2-min1 baseline)

- **📦 ConsoleBox 5 methods** - `log/warn/error/info/debug` の run-loop 最小実装
- **🧪 Build smoke** - `phase29cc_wsm_g2_min1_bridge_build_vm.sh` で契約固定
- **🎮 Interactive Playground** - `nyash_playground.html` の Run ボタンで最小デモ確認

## 📁 File Structure

```
projects/nyash-wasm/
├── README.md                 # This file
├── nyash_playground.html     # Interactive playground
├── build.sh                  # Build script
├── bridge/                   # wasm-pack target crate (SSOT)
└── pkg/                      # Generated WASM files (after build)
    ├── nyash_rust.js
    ├── nyash_rust_bg.wasm
    └── ...
```

## 🎨 Example Code (G2-min1 contract)

```nyash
local console = new ConsoleBox()
console.log("wsm02d_demo_min_log")
console.warn("wsm02d_demo_min_warn")
console.error("wsm02d_demo_min_error")
console.info("wsm02d_demo_min_info")
console.debug("wsm02d_demo_min_debug")
```

## 🔧 Development

### Build Process
1. `projects/nyash-wasm/bridge` を `wasm-pack` で build
2. `NyashWasm` を `pkg/nyash_rust.js` として export
3. `nyash_playground.html` から `NyashWasm.eval()` を呼び、ConsoleBox 5メソッドをブラウザ側へ出力
4. スモークで build/export/marker を fail-fast 固定

### Architecture
```
Browser JavaScript
    ↓
NyashWasm.eval(code)
    ↓ 
NyashInterpreter (Rust)
    ↓
ConsoleBox → web_sys::console
```

## 🎉 Coming Soon

- **DOMBox** - DOM manipulation from Nyash
- **CanvasBox** - Graphics and games
- **EventBox** - Mouse/keyboard event handling
- **HTTPBox** - Network requests
- **Sample Apps** - Snake game, Calculator, etc.

---

**Everything is Box, even in the browser! 🐱**
