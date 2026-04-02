# 🚀 Nyash WASM クイックスタート実装

## 現行の最短手順（WSM-G2 2026-02-26）

1. `bash projects/nyash-wasm/build.sh`
2. `bash tools/smokes/v2/profiles/integration/phase29cc_wsm/g2_browser/phase29cc_wsm_g2_min1_bridge_build_vm.sh`
3. `bash tools/smokes/v2/profiles/integration/phase29cc_wsm/g2_browser/phase29cc_wsm_g2_browser_run_vm.sh`
4. `tools/checks/dev_gate.sh wasm-demo-g2`
5. `tools/checks/dev_gate.sh wasm-demo-g3-core`（最小）または `tools/checks/dev_gate.sh wasm-demo-g3-full`（フル）

参照SSOT:
- `docs/development/current/main/phases/phase-29cc/29cc-134-wsm-g2-min1-bridge-run-loop-lock-ssot.md`
- `docs/development/current/main/phases/phase-29cc/29cc-135-wsm-g2-min2-headless-run-lock-ssot.md`

---

## 旧プロトタイプ手順（historical reference）

この節より下は旧プロトタイプの実装メモです。WASM/ブラウザ経路そのものは experimental / monitor-only の live lane ですが、以下の手順は current quick start ではありません。実験する場合は先頭の最短手順か `projects/nyash-wasm/README.md` を優先してください。

## Step 1: Cargo.toml修正

```toml
[dependencies]
wasm-bindgen = "0.2"
web-sys = "0.3"

[lib]
crate-type = ["cdylib", "rlib"]

[dependencies.web-sys]
version = "0.3"
features = [
  "console",
  "Document",
  "Element",
  "HtmlElement",
  "HtmlCanvasElement",
  "CanvasRenderingContext2d",
  "Window",
]
```

## Step 2: lib.rsにWASMエクスポート追加

```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct NyashWasm {
    interpreter: NyashInterpreter,
}

#[wasm_bindgen]
impl NyashWasm {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        // panicをconsole.errorに
        console_error_panic_hook::set_once();
        
        let mut interpreter = NyashInterpreter::new();
        // WASMBox等を登録
        Self { interpreter }
    }
    
    #[wasm_bindgen]
    pub fn eval(&mut self, code: &str) -> String {
        match self.interpreter.eval(code) {
            Ok(result) => format!("{:?}", result),
            Err(e) => format!("Error: {}", e),
        }
    }
}
```

## Step 3: ConsoleBox実装

```rust
// src/boxes/console_box.rs
pub struct ConsoleBox;

impl NyashBox for ConsoleBox {
    fn box_type(&self) -> &'static str { "ConsoleBox" }
    
    fn call_method(&self, name: &str, args: Vec<Arc<dyn NyashBox>>) -> Result<Arc<dyn NyashBox>, String> {
        match name {
            "log" => {
                let msg = args[0].to_string();
                web_sys::console::log_1(&msg.into());
                Ok(Arc::new(VoidBox))
            }
            _ => Err(format!("Unknown method: {}", name))
        }
    }
}
```

## Step 4: 簡単なHTML

```html
<!DOCTYPE html>
<html>
<head>
    <title>Nyash in Browser!</title>
    <style>
        #editor { width: 100%; height: 200px; }
        #output { border: 1px solid #ccc; padding: 10px; }
    </style>
</head>
<body>
    <h1>🐱 Nyash Browser Playground</h1>
    <textarea id="editor">
// Nyashコードをここに書くにゃ！
console = new ConsoleBox()
console.log("Hello from Nyash in Browser!")

x = 10
y = 20
console.log("x + y = " + (x + y))
    </textarea>
    <br>
    <button onclick="runNyash()">実行！</button>
    <div id="output"></div>
    
    <script type="module">
        import init, { NyashWasm } from './nyash_wasm.js';
        
        let nyash;
        
        async function main() {
            await init();
            nyash = new NyashWasm();
            window.runNyash = () => {
                const code = document.getElementById('editor').value;
                const output = nyash.eval(code);
                document.getElementById('output').textContent = output;
            };
        }
        
        main();
    </script>
</body>
</html>
```

## ビルドコマンド

```bash
# wasm-packインストール
cargo install wasm-pack

# ビルド
wasm-pack build --target web --out-dir www

# ローカルサーバー起動
cd www && python3 -m http.server 8000
```

これで http://localhost:8000 でNyashがブラウザで動く！🎉

## G2 ブラウザデモタスク（現行 2026-02-26）

1. `projects/nyash-wasm/build.sh` は `projects/nyash-wasm/bridge/` の独立 crate を build して `projects/nyash-wasm/pkg/` を更新する（ルート crate の wasm 互換性に依存しない）。
2. `bash tools/smokes/v2/profiles/integration/phase29cc_wsm/g2_browser/phase29cc_wsm_g2_min1_bridge_build_vm.sh` で build + export + playground marker を固定し、`ConsoleBox` の `log/warn/error/info/debug` 最小 run-loop を fail-fast で検証する。
3. `bash tools/smokes/v2/profiles/integration/phase29cc_wsm/g2_browser/phase29cc_wsm_g2_browser_run_vm.sh` で `autorun=1` headless chromium run を検証する。日常実行は `tools/checks/dev_gate.sh wasm-demo-g2` を使う。
4. 受け入れ記録と active next は `docs/development/current/main/phases/phase-29cc/README.md` を正本として参照する（この文書で列挙は持たない）。
