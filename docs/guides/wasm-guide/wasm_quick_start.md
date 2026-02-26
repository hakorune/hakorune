# 🚀 Nyash WASM クイックスタート実装（注意: 現状メンテ外）

このガイドは歴史的資料です。WASM/ブラウザ経路は現在メンテ対象外で、記載手順は最新のNyashと一致しない場合があります。実験する場合は `projects/nyash-wasm/` を参考に自己責任でお試しください。

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

1. `projects/nyash-wasm/build.sh` を使って wasm パッケージを再生成し、`nyash_playground.html` の Run ボタンが `ConsoleBox` の `log/warn/error/info/debug` を呼び出す最小デモであることを確認する。詳細な手順と acceptance wire が `docs/development/current/main/phases/phase-29cc/29cc-133-wsm-g2-browser-demo-task-plan.md` に記載している。
2. `python3 -m http.server` などで静的サーバーを立て、`nyash_playground.html` を headless ブラウザ（例: `node`+`playwright`）で開いて Run→コンソール出力を `wsm02d_demo_min_log` 〜 `wsm02d_demo_min_debug` で監視する smoke を整備する。出力マーキングは `apps/tests/phase29cc_wsm02d_demo_min.hako` と `tools/smokes/v2/profiles/integration/apps/phase29cc_wsm02d_demo_min_boundary_vm.sh` で gate 固定済みなので、この手順を docs で共有する。
3. headless smoke を `tools/checks/dev_gate.sh wasm-demo-g2` または類似の milestone entry へ追加し、`phase29cc_wsm02d_demo_min_*` の log マーカーで失敗を fail-fast にするとこの lane の acceptance 被りを防げる。
