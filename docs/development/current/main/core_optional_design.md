# Phase 103: CoreServices Optional化設計

## 概要

CoreServices（String/Integer/Bool/Array/Map/Console）を選択的に有効化し、メモリ制約環境対応。

## 設計方針

### 1. 環境変数制御

```bash
# デフォルト: すべて有効
./target/release/nyash app.hako

# StringService無効化
NYASH_CORE_DISABLE_STRING=1 ./target/release/nyash app.hako

# 複数無効化
NYASH_CORE_DISABLE_STRING=1 NYASH_CORE_DISABLE_INTEGER=1 ./target/release/nyash app.hako

# Minimal (ConsoleOnly)
NYASH_CORE_OPTIONAL=1 ./target/release/nyash app.hako
```

### 2. 初期化戦略

#### Default (全部有効)
```rust
let config = CoreServicesConfig::all_enabled();
```

#### Minimal (Console Only)
```rust
let config = CoreServicesConfig::minimal();
```

#### Custom (環境変数)
```rust
let config = CoreServicesConfig::from_env();
```

### 3. Fail-Safe原則

- **ConsoleBox**: 必須（ユーザー出力は絶対不可欠）
- **Others**: 無効可（アプリが必要なら呼び出し時Panic）
- **Fallback**: console_println! は必ずeprintln!フォールバック保持

### 4. 実装段階

| Phase | Scope | Status |
|-------|-------|--------|
| 103 | String/Integer/Bool/Array/Map Optional化 | 実装中 |
| 104 | ConsoleService も Optional化 + graceful error | Future |
| 105 | Memory pool設計 (MemApi と統合) | Future |

## 使用例

### ユースケース 1: 組み込み環境
```bash
# メモリ最小化
NYASH_CORE_DISABLE_ARRAY=1 NYASH_CORE_DISABLE_MAP=1 ./target/release/nyash app.hako
```

### ユースケース 2: Web Worker
```bash
# GCオーバーヘッド削減（StringBox無効）
NYASH_CORE_DISABLE_STRING=1 ./target/release/nyash app.hako
```

## CoreBoxesImpl の型変更

Phase 103 で以下の変更を実施:

```rust
// 変更前
pub struct CoreServices {
    pub string: Arc<dyn StringService>,
    pub integer: Arc<dyn IntegerService>,
    // ... (Required)
}

// 変更後
pub struct CoreServices {
    pub string: Option<Arc<dyn StringService>>,
    pub integer: Option<Arc<dyn IntegerService>>,
    // ... (Optional)
}
```

## 次フェーズ (104)

Phase 104 では ConsoleService も optional 対応し、完全な graceful degradation 実現予定。

```rust
// Future: console_println! graceful fallback
#[macro_export]
macro_rules! console_println {
    ($($arg:tt)*) => {
        if let Some(host) = $crate::runtime::try_get_core_plugin_host() {
            if let Some(console) = &host.core.console {
                console.println(&format!($($arg)*));
            } else {
                eprintln!($($arg)*);  // Fallback
            }
        } else {
            eprintln!($($arg)*);  // Fallback
        }
    };
}
```
