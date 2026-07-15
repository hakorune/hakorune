# BID-FFI v1 実装仕様書 (実装ベース)

## 🎯 概要

**これは現在動作している実装をベースとした正確な仕様書です。**
- FileBoxプラグインで実証済み
- plugin_loader_v2.rsの実装に基づく
- 理想案ではなく、実際に動く仕様

> Vocabulary boundary: `clone_box()` / `share_box()` はこのSharedV1/plugin
> runtime ABIの名前であり、Hakorune source ownershipの意味を決めません。
> source契約は `docs/reference/language/ownership.md` が正本です。

## 📋 プラグインAPI仕様

### 必須エクスポート関数

#### 1. ABI Version (オプション)
```c
extern "C" u32 nyash_plugin_abi(void) {
    return 1;  // BID-FFI v1
}
```

#### 2. 初期化 (オプション)
```c
extern "C" i32 nyash_plugin_init(void) {
    // グローバルリソース初期化
    // 0=成功, 負数=エラー（プラグイン無効化）
    return 0;
}
```

#### 3. メソッド呼び出し (必須)
```c
extern "C" i32 nyash_plugin_invoke(
    u32 type_id,      // Box型ID (6=FileBox)
    u32 method_id,    // メソッドID (0=birth, 4294967295=fini)
    u32 instance_id,  // インスタンスID (0=static call)
    const u8* args,   // TLV引数
    usize args_len,   // 引数サイズ
    u8* result,       // TLV結果バッファ
    usize* result_len // [IN/OUT]バッファサイズ
) -> i32;             // 0=成功, 負数=エラー
```

#### 4. 終了処理 (オプション)
```c
extern "C" void nyash_plugin_shutdown(void) {
    // グローバルリソース解放
}
```

## 📊 エラーコード

```c
#define NYB_SUCCESS           0   // 成功
#define NYB_E_SHORT_BUFFER   -1   // バッファ不足
#define NYB_E_INVALID_TYPE   -2   // 無効な型ID
#define NYB_E_INVALID_METHOD -3   // 無効なメソッドID
#define NYB_E_INVALID_ARGS   -4   // 無効な引数
#define NYB_E_PLUGIN_ERROR   -5   // プラグイン内部エラー
#define NYB_E_INVALID_HANDLE -8   // 無効なハンドル
```

## 🏗️ TLV (Type-Length-Value) 形式

### ヘッダー構造
```c
struct TlvHeader {
    u16 version;  // 1 (BID-FFI v1)
    u16 argc;     // 引数数
};
```

### エントリー構造
```c
struct TlvEntry {
    u8 tag;       // 型タグ
    u8 reserved;  // 0（将来拡張用）
    u16 size;     // ペイロードサイズ
    // followed by payload data
};
```

### 型タグ定義
```c
#define BID_TAG_BOOL    1   // bool: 1 byte (0/1)
#define BID_TAG_I32     2   // i32: 4 bytes (little-endian)
#define BID_TAG_I64     3   // i64: 8 bytes (little-endian)
#define BID_TAG_F32     4   // f32: 4 bytes (IEEE 754)
#define BID_TAG_F64     5   // f64: 8 bytes (IEEE 754)
#define BID_TAG_STRING  6   // string: UTF-8 bytes
#define BID_TAG_BYTES   7   // bytes: binary data
#define BID_TAG_HANDLE  8   // handle: 8 bytes (type_id + instance_id)
#define BID_TAG_VOID    9   // void: 0 bytes
```

## 🔧 nyash.toml設定仕様

### 基本構造
```toml
[libraries."<library_name>"]
boxes = ["BoxType1", "BoxType2"]  # 提供するBox型
path = "./path/to/library.so"     # ライブラリパス

[libraries."<library_name>".<BoxType>]
type_id = <number>  # Box型ID (必須)

[libraries."<library_name>".<BoxType>.methods]
<method_name> = { method_id = <number> }
```

### 実例 (FileBox)
```toml
[libraries."libnyash_filebox_plugin.so"]
boxes = ["FileBox"]
path = "./plugins/nyash-filebox-plugin/target/release/libnyash_filebox_plugin.so"

[libraries."libnyash_filebox_plugin.so".FileBox]
type_id = 6

[libraries."libnyash_filebox_plugin.so".FileBox.methods]
birth = { method_id = 0 }          # コンストラクタ
open = { method_id = 1 }
read = { method_id = 2 }
write = { method_id = 3 }
close = { method_id = 4 }
fini = { method_id = 4294967295 }  # デストラクタ (u32::MAX)
```

## 🔄 必須メソッド規約

### birth() - コンストラクタ
- **method_id**: 必ず 0
- **引数**: TLV形式（型依存）
- **戻り値**: instance_id (u32, little-endian, 4bytes)
- **呼び出し**: instance_id=0 (static call)

### fini() - デストラクタ  
- **method_id**: 必ず 4294967295 (u32::MAX)
- **引数**: 空のTLV (version=1, argc=0)
- **戻り値**: Void
- **呼び出し**: 対象のinstance_id

## 📝 PluginBoxV2構造体

```rust
pub struct PluginBoxV2 {
    pub box_type: String,              // "FileBox"
    pub type_id: u32,                  // 6
    pub invoke_fn: InvokeFn,           // 関数ポインタ
    pub instance_id: u32,              // プラグイン生成ID
    pub fini_method_id: Option<u32>,   // finiメソッドID
}
```

## 🚨 重要な制約

### メモリ管理
- **プラグイン責任**: プラグインが確保したメモリはプラグインが解放
- **2段階呼び出し**: 
  1. result=NULL でサイズ取得
  2. ホストがバッファ確保後、実際のデータ取得

### 文字列エンコーディング
- **UTF-8必須**: すべての文字列はUTF-8
- **NUL終端不要**: lengthが正確性を保証

### インスタンス管理
- **instance_id**: プラグイン内で一意
- **birth順序**: birth() → 実際のメソッド → fini()
- **共有・複製**: clone_box()は新birth()、share_box()は同一instance_id

## 🔗 実装ファイル

### Nyash側
- `src/runtime/plugin_loader_v2.rs` - プラグインローダー
- `src/config/nyash_toml_v2.rs` - 設定パーサー
- `src/bid/tlv.rs` - TLVエンコーダー/デコーダー

### プラグイン例
- `plugins/nyash-filebox-plugin/src/lib.rs` - FileBox実装
- `plugins/nyash-test-multibox/src/lib.rs` - マルチBox実装

## ✅ 動作確認済み

- ✅ FileBoxプラグイン完全動作
- ✅ birth/finiライフサイクル
- ✅ TLVエンコーディング/デコーディング
- ✅ clone_box/share_box メソッド
- ✅ マルチインスタンス管理

---

**最終更新**: 2025年8月20日 - Phase 1現実調査完了  
**ベース**: plugin_loader_v2.rs実装 + FileBox実証  
**状態**: Production Ready (実際に動作中)
