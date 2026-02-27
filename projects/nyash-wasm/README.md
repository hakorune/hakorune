# 🌐 Nyash WebAssembly Project

Status: `WSM-G4-min6` まで固定済み。`projects/nyash-wasm/bridge` を wasm build のSSOTとし、`nyash_playground.html` の Console baseline / canvas primer / webcanvas+canvas_advanced の fixture+headless parity を lock 済み。wasm lane は monitor-only。

## 🚀 Quick Start (experimental)

```bash
# Install wasm-pack (if not already installed)
cargo install wasm-pack

# Build WASM module (bridge crate)
bash projects/nyash-wasm/build.sh

# Build bridge + static prebuilt wasm demos
bash projects/nyash-wasm/build.sh

# Start static server
cd projects/nyash-wasm && python3 -m http.server 8001

# Open browser
# Navigate to: http://localhost:8001/nyash_playground.html
```

## 🎯 Features (G4-min1 baseline)

- **📦 ConsoleBox 5 methods** - `log/warn/error/info/debug` の run-loop 最小実装
- **🧪 Build smoke** - `phase29cc_wsm_g2_min1_bridge_build_vm.sh` で契約固定
- **🧪 Migration baseline smoke** - `phase29cc_wsm_g4_min1_playground_console_baseline_vm.sh` で run loop + fixture parity を固定
- **🧪 Canvas primer smoke** - `phase29cc_wsm_g4_min2_playground_canvas_primer_vm.sh` で primer vocabulary を固定
- **🧪 Webcanvas fixture parity smoke** - `phase29cc_wsm_g4_min3_webcanvas_fixture_parity_vm.sh`
- **🧪 Canvas advanced fixture parity smoke** - `phase29cc_wsm_g4_min4_canvas_advanced_fixture_parity_vm.sh`
- **🧪 Headless two-example parity smoke** - `phase29cc_wsm_g4_min5_headless_two_examples_vm.sh`
- **🧪 G4 closeout smoke** - `phase29cc_wsm_g4_min6_gate_promotion_closeout_vm.sh`
- **🎮 Interactive Playground** - `nyash_playground.html` の Run ボタンで prebuilt demo 実行

## 📁 File Structure

```
projects/nyash-wasm/
├── README.md                 # This file
├── nyash_playground.html     # Interactive playground
├── build.sh                  # Build script
├── prebuilt/                 # Static prebuilt wasm demos for playground
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
3. `apps/tests/*` fixture から `prebuilt/*.wasm` を生成
4. `nyash_playground.html` は prebuilt wasm を読み込んで実行
5. スモークで build/export/marker を fail-fast 固定

### Architecture
```
Browser JavaScript
    ↓
fetch ./prebuilt/*.wasm
    ↓
WebAssembly.instantiate + env imports
```

## 📌 Operation Policy

- 配信は static-first（`python -m http.server` / GitHub static hosting）を基本とする。
- `prebuilt/` 生成物は playground配信契約の一部としてコミット運用する。
- ソース更新時は `bash projects/nyash-wasm/build.sh` を実行し、`pkg/` と `prebuilt/` を同期する。

## 🎉 Coming Soon

- **DOMBox** - DOM manipulation from Nyash
- **CanvasBox** - Graphics and games
- **EventBox** - Mouse/keyboard event handling
- **HTTPBox** - Network requests
- **Sample Apps** - Snake game, Calculator, etc.

---

**Everything is Box, even in the browser! 🐱**
