# 📦 Nyash ビルトインBox → プラグイン化移行ガイド v2

## 🎯 概要
NyashのビルトインBoxをプラグイン化し、コアを軽量化します。
FileBoxプラグインの成功例を詳しく解説しながら、移行方法を説明します。

## 🔑 重要な概念：nyash.tomlの型定義システム

### 型変換の仕組み
nyash.tomlでは、Nyash側とプラグイン側の型変換を明示的に定義します：

```toml
# FileBoxの例
[plugins.FileBox.methods]
# writeメソッド：Nyashのstringをプラグインではbytesとして扱う
write = { args = [{ from = "string", to = "bytes" }] }

# openメソッド：2つのstring引数（型変換なし）
open = { args = [
    { name = "path", from = "string", to = "string" },
    { name = "mode", from = "string", to = "string" }
] }
```

### from/toの意味
- **from**: Nyash側の型（ユーザーが渡す型）
- **to**: プラグイン側で受け取る型（TLVエンコーディング）

### TLVタグとの対応
プラグインはTLV（Type-Length-Value）形式でデータを受け取ります：
- `to = "i32"` → TLV tag=2（32ビット整数）
- `to = "string"` → TLV tag=6（UTF-8文字列）
- `to = "bytes"` → TLV tag=7（バイト配列）

## 📋 移行対象Box一覧（優先順位順）

### 🌐 Phase 1: ネットワーク系（最優先・最も簡単）
既にスタブ実装があり、reqwest依存を追加するだけで完成します。

#### HttpClientBox
```toml
[plugins.HttpClientBox.methods]
# シンプルなGETリクエスト
get = { 
    args = [{ from = "string", to = "string" }],  # URL
    returns = "string"  # レスポンスボディ
}

# POSTリクエスト（ボディ付き）
post = { 
    args = [
        { from = "string", to = "string" },  # URL
        { from = "string", to = "bytes" }    # ボディ（バイナリ対応）
    ],
    returns = "string"
}

# 詳細なリクエスト（ヘッダー等を含む）
request = {
    args = [
        { from = "string", to = "string" },  # メソッド（GET/POST等）
        { from = "string", to = "string" },  # URL
        { from = "map", to = "map" }         # オプション（headers, timeout等）
    ],
    returns = "map"  # { status: i32, body: string, headers: map }
}
```

### 🖼️ Phase 2: GUI系（プラットフォーム依存）
EguiBoxは既にfeature分離されているので参考になります。

### 🎵 Phase 3: 特殊用途系（独立性高い）
TimerBox、QRBox等は単機能で実装しやすいです。

## 🔧 実装ガイドライン

### 1. 参考にするファイル
- **成功例**: `plugins/nyash-filebox-plugin/` - 動作確認済みのFileBoxプラグイン
- **設定例**: `nyash.toml` - 型情報定義の書き方
- **テスト**: `tools/plugin-tester/` - プラグイン診断ツール

### 2. 各プラグインの構成
```
plugins/nyash-xxx-plugin/
├── Cargo.toml      # 依存関係（例: reqwest for HTTP）
├── src/
│   └── lib.rs      # FFI実装
├── nyash.toml      # 型情報定義
└── README.md       # 使用方法
```

### 3. nyash.toml記述例（HttpClientBoxの場合）
```toml
[plugins.HttpClientBox.methods]
# GETリクエスト
get = { 
    args = [{ name = "url", from = "string", to = "string" }],
    returns = "string"
}

# POSTリクエスト
post = { 
    args = [
        { name = "url", from = "string", to = "string" },
        { name = "body", from = "string", to = "string" }
    ],
    returns = "string"
}

# ヘッダー付きリクエスト
request = {
    args = [
        { name = "method", from = "string", to = "string" },
        { name = "url", from = "string", to = "string" },
        { name = "options", from = "map", to = "map" }
    ],
    returns = "map"  # { status, body, headers }
}

# DELETE リクエスト
delete = {
    args = [{ name = "url", from = "string", to = "string" }],
    returns = "string"
}

# PUT リクエスト  
put = {
    args = [
        { name = "url", from = "string", to = "string" },
        { name = "body", from = "string", to = "string" }
    ],
    returns = "string"
}
```

### 4. テスト方法
```bash
# ビルド
cd plugins/nyash-xxx-plugin
cargo build --release

# plugin-testerで診断
cd ../../tools/plugin-tester
./target/release/plugin-tester ../../plugins/nyash-xxx-plugin/target/release/libnyash_xxx_plugin.so

# Nyashで実行テスト
./target/release/nyash test_xxx.hako
```

## 📝 特記事項

### HttpBox系
- 現在スタブ実装なので移行しやすい
- reqwest依存を復活させる
- 非同期処理の考慮が必要

### EguiBox
- 既にfeature分離されているので参考になる
- メインスレッド制約に注意

### AudioBox/SoundBox
- プラットフォーム依存性が高い
- Web/Desktop両対応を検討

### 依存関係の管理
- 各プラグインは独立したCargo.tomlを持つ
- ビルド時間短縮のため最小限の依存にする

## 💡 実装の重要ポイント

### FFI境界での注意事項
1. **メモリ管理**: 
   - Rustの所有権とCのメモリ管理の違いに注意
   - 文字列は必ずCString/CStr経由で変換
   
2. **エラーハンドリング**:
   - パニックをFFI境界で止める（catch_unwind使用）
   - エラーコードで通信（0=成功, 負値=エラー）

3. **型変換パターン** (FileBoxプラグインより):
```rust
// Nyash文字列 → Rust文字列
let path = get_string_arg(&args[0], 0)?;

// Rust文字列 → Nyash文字列
encode_string_result(&contents, result, result_len)
```

### 参考ファイルの具体的パス
- **FileBoxプラグイン実装**: `plugins/nyash-filebox-plugin/src/lib.rs`
- **FFI仕様書**: `docs/説明書/reference/plugin-system/ffi-abi-specification.md`
- **プラグインシステム説明**: `docs/説明書/reference/plugin-system/plugin-system.md`
- **BID-FFI型変換** (参考): `src/bid-converter-copilot/tlv.rs`

## 🔧 実装ガイド：FileBoxを例に

### 1. プラグイン側での型受け取り例

```rust
// nyash.toml: write = { args = [{ from = "string", to = "bytes" }] }
METHOD_WRITE => {
    // TLVでbytesとして受け取る
    let data = tlv_parse_bytes(args)?;  // Vec<u8>として取得
    
    // ファイルに書き込み
    match file.write(&data) {
        Ok(n) => {
            file.flush()?;  // 重要：フラッシュを忘れずに！
            // 書き込んだバイト数を返す（TLV i32）
            write_tlv_i32(n as i32, result, result_len)
        }
        Err(_) => NYB_E_PLUGIN_ERROR
    }
}
```

### 2. 複数引数の解析例

```rust
// nyash.toml: open = { args = [{ from = "string", to = "string" }, { from = "string", to = "string" }] }
METHOD_OPEN => {
    // 2つのstring引数を解析
    let (path, mode) = tlv_parse_two_strings(args)?;
    
    // ファイルを開く
    let file = match mode.as_str() {
        "r" => File::open(&path)?,
        "w" => File::create(&path)?,
        "a" => OpenOptions::new().append(true).open(&path)?,
        _ => return NYB_E_INVALID_ARGS
    };
    
    // 成功時はVoid（空）を返す
    write_tlv_void(result, result_len)
}
```

### 3. 引数なしメソッドの例

```rust
// nyash.toml: read = { args = [] }
METHOD_READ => {
    // 引数なし - ファイル全体を読む
    file.seek(SeekFrom::Start(0))?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;
    
    // bytesとして返す
    write_tlv_bytes(&buf, result, result_len)
}
```

## 📝 HttpClientBox実装の具体例

```rust
// HttpClientBoxプラグインの実装イメージ
use reqwest::blocking::Client;

METHOD_GET => {
    // URLを解析
    let url = tlv_parse_string(args)?;
    
    // HTTPリクエスト実行
    let client = Client::new();
    let response = client.get(&url).send()?;
    let body = response.text()?;
    
    // 文字列として返す
    write_tlv_string(&body, result, result_len)
}

METHOD_POST => {
    // URL と ボディを解析
    let (url, body_bytes) = tlv_parse_string_and_bytes(args)?;
    
    // POSTリクエスト
    let client = Client::new();
    let response = client.post(&url)
        .body(body_bytes)
        .send()?;
    let body = response.text()?;
    
    write_tlv_string(&body, result, result_len)
}
```

## 💡 実装のコツとよくある間違い

### ✅ 正しいnyash.toml
```toml
# 引数の型変換を明示
write = { args = [{ from = "string", to = "bytes" }] }

# 戻り値の型も指定可能
exists = { args = [], returns = "bool" }
```

### ❌ よくある間違い
```toml
# 間違い：型情報がない
write = { args = ["string"] }  # ❌ from/toが必要

# 間違い：不要なフィールド
get = { args = [{ type = "string" }] }  # ❌ typeではなくfrom/to
```

### メモリ管理の注意点
1. 文字列は必ずCString/CStr経由で変換
2. プラグイン側でallocしたメモリはプラグイン側でfree
3. ホスト側のVtableを使ってログ出力

### エラーハンドリング
```rust
// パニックをFFI境界で止める
let result = std::panic::catch_unwind(|| {
    // 実際の処理
});

match result {
    Ok(val) => val,
    Err(_) => NYB_E_PLUGIN_ERROR
}
```

## 🧪 テスト方法

### 1. プラグインビルド
```bash
cd plugins/nyash-http-plugin
cargo build --release
```

### 2. plugin-testerで診断
```bash
cd ../../tools/plugin-tester
./target/release/plugin-tester ../../plugins/nyash-http-plugin/target/release/libnyash_http_plugin.so

# 期待される出力：
# Plugin Information:
#   Box Type: HttpClientBox (ID: 20)
#   Methods: 5
#   - birth [ID: 0] (constructor)
#   - get, post, put, delete
#   - fini [ID: 4294967295] (destructor)
```

### 3. Nyashで実行
```nyash
// test_http.hako
local http = new HttpClientBox()
local response = http.get("https://api.example.com/data")
print(response)
```

## 📚 参考資料
- **FileBoxプラグイン完全実装**: `plugins/nyash-filebox-plugin/src/lib.rs`
- **TLVエンコーディング仕様**: `docs/説明書/reference/plugin-system/ffi-abi-specification.md`
- **nyash.toml設定例**: プロジェクトルートの`nyash.toml`

## 🎯 成功の秘訣
1. **FileBoxを完全に理解してから始める** - コピペベースで改造
2. **nyash.tomlの型定義を正確に** - from/toを明示
3. **TLVの理解** - tag=6(string), tag=7(bytes)の違い
4. **plugin-testerで早期検証** - 問題を早期発見

---

質問があれば、FileBoxの実装を参考にしてください。
すべての答えがそこにあります！