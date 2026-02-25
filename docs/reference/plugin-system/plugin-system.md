# Nyash Box プラグインシステム設計

> ⚠️ **DEPRECATED - 将来構想**
> 
> この文書はYAML DSLを使った将来的なプラグインシステム構想です。
> **現在の実装とは異なります。**
> 
> **実際に動作している仕様については、以下を参照してください：**
> - [BID-FFI v1 実装仕様書](./bid-ffi-v1-actual-specification.md) - 現在動作中の仕様
> - [nyash.toml設定例](../../../../nyash.toml) - 実際の設定形式
> - [plugin_loader_v2.rs](../../../../src/runtime/plugin_loader_v2.rs) - 実装詳細

## 概要

Nyashの「Everything is Box」哲学を維持しながら、Boxの実装をプラグイン化できるシステム。ビルトインBoxとプラグインBoxを透過的に切り替え可能。

## 🎯 設計原則

1. **シンプル** - 設定ファイル1つで切り替え
2. **透過的** - Nyashコードの変更不要
3. **統一的** - ビルトインもプラグインも同じBox

## 📋 プラグイン定義（YAML署名DSL）

```yaml
# filebox.plugin.yaml
schema: 1
plugin:
  name: filebox
  version: 1
  
apis:
  # 静的メソッド（::）
  - sig: "FileBox::open(path: string, mode?: string) -> FileBox"
    doc: "Open a file with optional mode"
    
  - sig: "FileBox::exists(path: string) -> bool"
    doc: "Check if file exists"
    
  # インスタンスメソッド（#）
  - sig: "FileBox#read(size?: int) -> string"
    doc: "Read file content"
    
  - sig: "FileBox#write(content: string) -> int"
    doc: "Write to file"
    
  - sig: "FileBox#close() -> void"
    doc: "Close file handle"
```

### 署名DSL仕様

- **静的メソッド**: `Type::method()` - C++風の`::`記法
- **インスタンスメソッド**: `Type#method()` - Ruby風の`#`記法
- **オプショナル引数**: `arg?: type` - `?`サフィックス
- **戻り値**: `-> type` - 矢印記法

### 🔄 Boxライフサイクル管理

```yaml
lifecycle:
  # コンストラクタ（生命を与える）
  - sig: "FileBox#birth(path: string, mode?: string)"
    doc: "Box creation - called after memory allocation"
    
  # デストラクタ（生命を終える）  
  - sig: "FileBox#fini()"
    doc: "Box destruction - called before memory deallocation"
```

**重要な原則**：
- `birth()` - Boxインスタンス作成時に呼ばれる（メモリ割り当て後）
- `fini()` - Boxインスタンス破棄時に呼ばれる（メモリ解放前）
- プラグインが割り当てたメモリはプラグインが解放する責任を持つ

## 🔧 設定ファイル（nyash.toml）

### 基本形式（v1） - 単一Box型プラグイン

```toml
# プロジェクトルートのnyash.toml
[plugins]
FileBox = "nyash-filebox-plugin"      # FileBoxはプラグイン版を使用
# StringBox = "mystring"                # コメントアウト = ビルトイン使用

# FileBoxの型情報定義
[plugins.FileBox.methods]
read = { args = [] }
write = { args = [{ from = "string", to = "bytes" }] }
open = { args = [
    { name = "path", from = "string", to = "string" },
    { name = "mode", from = "string", to = "string" }
] }
close = { args = [] }
exists = { args = [], returns = "bool" }
```

### 拡張形式（v2） - マルチBox型プラグイン

```toml
# 1つのプラグインで複数のBox型を提供
[plugins.libraries]
"nyash-network" = {
    plugin_path = "libnyash_network.so",
    provides = ["SocketBox", "HTTPServerBox", "HTTPRequestBox", "HTTPResponseBox", "HttpClientBox"]
}

"nyash-stdlib" = {
    plugin_path = "libnyash_stdlib.so",
    provides = ["MathBox", "TimeBox", "RandomBox"]
}

# 各Box型の詳細定義
[plugins.types.SocketBox]
library = "nyash-network"
type_id = 100
methods = {
    bind = { args = [
        { name = "address", from = "string", to = "string" },
        { name = "port", from = "integer", to = "u16" }
    ]},
    connect = { args = [
        { name = "address", from = "string", to = "string" },
        { name = "port", from = "integer", to = "u16" }
    ]},
    read = { args = [], returns = "string" },
    write = { args = [{ from = "string", to = "bytes" }] },
    close = { args = [] }
}

[plugins.types.HTTPServerBox]
library = "nyash-network"
type_id = 101
methods = {
    bind = { args = [
        { name = "address", from = "string", to = "string" },
        { name = "port", from = "integer", to = "u16" }
    ]},
    route = { args = [
        { name = "path", from = "string", to = "string" },
        { name = "method", from = "string", to = "string" }
    ]},
    start = { args = [] }
}

[plugins.types.HttpClientBox]
library = "nyash-network"
type_id = 102
methods = {
    get = { args = [{ name = "url", from = "string", to = "string" }], returns = "string" },
    post = { args = [
        { name = "url", from = "string", to = "string" },
        { name = "body", from = "string", to = "string" }
    ], returns = "string" }
}
```

### 型マッピング仕様

#### 基本型
| Nyash型 | FFI型 | TLVタグ | 説明 |
|---------|-------|---------|------|
| `string` | `string` | 0x01 | UTF-8文字列 |
| `integer` | `i64` | 0x02 | 64ビット整数 |
| `float` | `f64` | 0x03 | 64ビット浮動小数点 |
| `bool` | `bool` | 0x04 | 真偽値 |
| `bytes` | `Vec<u8>` | 0x05 | バイト配列 |


### プラグイン検索パス

```toml
[plugin_paths]
search_paths = [
    "./plugins/*/target/release",      # 開発時リリースビルド
    "./plugins/*/target/debug",        # 開発時デバッグビルド
    "/usr/local/lib/nyash/plugins",    # システムインストール
    "~/.hako/plugins"                 # ユーザーローカル
]
```

## 🏗️ アーキテクチャ

### 1. Boxレジストリ（v2対応版）

```rust
// 起動時の動作
let mut registry = HashMap::new();
let mut loaded_plugins = HashMap::new();

// 1. ビルトインBoxを登録
registry.insert("FileBox", BoxProvider::Builtin(native_filebox));
registry.insert("StringBox", BoxProvider::Builtin(native_stringbox));

// 2. nyash.toml読み込み
let config = parse_nyash_toml_v2()?;

// 3a. v1形式：単一Box型プラグイン
for (box_name, plugin_name) in config.plugins {
    registry.insert(box_name, BoxProvider::Plugin(plugin_name));
}

// 3b. v2形式：マルチBox型プラグイン
if let Some(libraries) = config.libraries {
    for (lib_name, lib_def) in libraries.libraries {
        // プラグインを一度だけロード
        let plugin = load_plugin(&lib_def.plugin_path)?;
        loaded_plugins.insert(lib_name.clone(), plugin);
        
        // 提供する全Box型を登録
        for box_type in &lib_def.provides {
            registry.insert(box_type, BoxProvider::MultiPlugin(lib_name.clone()));
        }
    }
}
```

### マルチBox型プラグインFFI

```c
// v2プラグインの追加エクスポート関数
// 提供するBox型の数を返す
extern "C" u32 nyash_plugin_get_box_count();

// 各Box型の情報を取得
extern "C" NyashPluginInfo* nyash_plugin_get_box_info(u32 index);

// Box型名からtype_idを解決
extern "C" u32 nyash_plugin_get_type_id(const char* box_name);
```

### 2. 透過的なディスパッチ

```nyash
# Nyashコード（変更不要！）
local file = new FileBox("test.txt")
file.write("Hello, plugin!")
local content = file.read()
```

内部動作:
1. `new FileBox` → レジストリ検索
2. `BoxProvider::Plugin("filebox")` → プラグインロード
3. BID-FFI経由で実行

### 3. PluginBoxプロキシ

```rust
// すべてのプラグインBoxの統一インターフェース
pub struct PluginBox {
    plugin_name: String,
    handle: BidHandle,  // プラグイン内のインスタンス
}

impl NyashBox for PluginBox {
    // NyashBoxトレイトの全メソッドを
    // FFI経由でプラグインに転送
}
```

## 📦 プラグイン実装例

```c
// plugins/filebox/src/filebox.c
#include "nyash_plugin_api.h"

// インスタンス管理
typedef struct {
    FILE* fp;
    char* buffer;  // プラグインが管理するバッファ
} FileBoxInstance;

// birth - Boxに生命を与える
i32 filebox_birth(u32 instance_id, const u8* args, size_t args_len) {
    // 引数からpath, modeを取得
    const char* path = extract_string_arg(args, 0);
    const char* mode = extract_string_arg(args, 1);
    
    // インスタンス作成
    FileBoxInstance* instance = malloc(sizeof(FileBoxInstance));
    instance->fp = fopen(path, mode);
    instance->buffer = NULL;
    
    // インスタンスを登録
    register_instance(instance_id, instance);
    return NYB_SUCCESS;
}

// fini - Boxの生命を終える
i32 filebox_fini(u32 instance_id) {
    FileBoxInstance* instance = get_instance(instance_id);
    if (!instance) return NYB_E_INVALID_HANDLE;
    
    // プラグインが割り当てたメモリを解放
    if (instance->buffer) {
        free(instance->buffer);
    }
    
    // ファイルハンドルをクローズ
    if (instance->fp) {
        fclose(instance->fp);
    }
    
    // インスタンス自体を解放
    free(instance);
    unregister_instance(instance_id);
    
    return NYB_SUCCESS;
}

// read - バッファはプラグインが管理
i32 filebox_read(u32 instance_id, i32 size, u8** result, size_t* result_len) {
    FileBoxInstance* instance = get_instance(instance_id);
    
    // 既存バッファを解放して新規割り当て
    if (instance->buffer) free(instance->buffer);
    instance->buffer = malloc(size + 1);
    
    // ファイル読み込み
    size_t read = fread(instance->buffer, 1, size, instance->fp);
    instance->buffer[read] = '\0';
    
    // プラグインが所有するメモリを返す
    *result = instance->buffer;
    *result_len = read;
    
    return NYB_SUCCESS;
}
```

## 🔐 メモリ管理の原則

### 所有権ルール
1. **プラグインが割り当てたメモリ**
   - プラグインが`malloc()`したメモリはプラグインが`free()`する
   - `fini()`メソッドで確実に解放する
   - Nyash側は読み取りのみ（書き込み禁止）

2. **Nyashが割り当てたメモリ**
   - Nyashが提供したバッファはNyashが管理
   - プラグインは読み書き可能だが解放禁止
   - 引数として渡されたメモリはread-only

3. **ライフサイクル保証**
   - `birth()` → 各メソッド呼び出し → `fini()` の順序を保証
   - `fini()` は論理的終了（use-after-fini禁止）。自動呼び出しは実行経路/所有形態に依存しうるため、必要な資源（fd/socket等）は明示 `fini()` / `shutdown_plugins_v2()` で確実に解放する
   - 循環参照や共有（複数スコープに跨る参照）では `fini()` タイミングが遅延/未観測になりうるため、weak/singleton/明示finiで設計する
   - SSOT: `docs/reference/language/lifecycle.md`

### Nyash側の実装
```rust
impl Drop for PluginBox {
    fn drop(&mut self) {
        // 破棄時の best-effort cleanup（実行経路/所有形態によりタイミングは変わりうる）。
        // 言語仕様としては `fini()` は明示的に呼ぶ/`shutdown_plugins_v2()` で閉じるのが推奨。
        let result = self.plugin.invoke(
            self.handle.type_id,
            FINI_METHOD_ID,  // 最大値のmethod_id
            self.handle.instance_id,
            &[],  // no arguments
            &mut []
        );
        
        if result.is_err() {
            eprintln!("Warning: fini failed for instance {}", self.handle.instance_id);
        }
    }
}
```

## 🚀 段階的導入計画

### Phase 1: 基本実装（完了）
- [x] BID-FFI基盤
- [x] FileBoxプラグイン実装
- [x] nyash.toml v1パーサー
- [x] PluginBoxプロキシ
- [x] プラグインロード機能

### Phase 2: マルチBox型対応（進行中）
- [ ] nyash.toml v2パーサー実装
- [ ] マルチBox型プラグインFFI拡張
- [ ] plugin-testerの複数Box型対応
- [ ] ネットワーク系プラグイン統合
  - HttpClientBox（新規実装）
  - SocketBox（既存移行）
  - HTTPServerBox（既存移行）
  - HTTPRequestBox（既存移行）
  - HTTPResponseBox（既存移行）

### Phase 3: 開発体験向上
- [ ] YAMLからFFIコード自動生成
- [ ] エラーメッセージ改善
- [ ] プラグインテンプレート
- [ ] ホットリロード対応

### Phase 4: エコシステム
- [ ] プラグインレジストリ
- [ ] バージョン管理
- [ ] 依存関係解決
- [ ] プラグイン間通信

## 🎉 利点

### v1形式の利点
1. **ビルド時間短縮** - 使わないBoxはコンパイル不要
2. **動的拡張** - 再コンパイルなしで新Box追加
3. **Everything is Box維持** - 哲学は変わらない
4. **段階的移行** - 1つずつBoxをプラグイン化

### v2形式の追加利点
5. **依存関係の解決** - 関連Box群を1つのプラグインに
6. **効率的な配布** - 複数Box型を1ライブラリで提供
7. **メモリ効率** - 共有ライブラリは1度だけロード
8. **内部連携** - 同一プラグイン内で直接通信可能

### 実例：HTTPServerBoxの依存問題解決

```toml
# v1では困難だった構成
# HTTPServerBoxはSocketBoxに依存するが...
[plugins]
SocketBox = "socket-plugin"      # 別プラグイン
HTTPServerBox = "http-plugin"    # SocketBoxが使えない！

# v2なら簡単に解決
[plugins.libraries]
"nyash-network" = {
    plugin_path = "libnyash_network.so",
    provides = ["SocketBox", "HTTPServerBox", "HTTPRequestBox", "HTTPResponseBox"]
}
# HTTPServerBoxは同じプラグイン内でSocketBoxを直接使用可能
```

## 📚 関連ドキュメント

- [BID-FFI仕様](./ffi-abi-specification.md)
- [Everything is Box哲学](./everything-is-box.md)
- [実装タスク](../../../予定/native-plan/issues/phase_9_75g_0_chatgpt_enhanced_final.md)
