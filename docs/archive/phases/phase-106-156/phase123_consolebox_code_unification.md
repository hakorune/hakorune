# Phase 123: ConsoleBox WASM/非WASM コード統一化

⚠️ **Note**: このドキュメントは Phase 123 の実装記録です。
           統合的なガイドは [ConsoleBox 完全ガイド](consolebox_complete_guide.md) をご参照ください。

## 目的

`src/boxes/console_box.rs` で WASM/非WASM 環境で分離・重複している実装をマクロを使用して統一し、保守性と可読性を向上させる。

現在 ~245行の実装を ~180行程度に削減（~25%削減見込み）

## 現在の問題

### コード重複の内容

**src/boxes/console_box.rs の現状**:

```
📦 WASM版（行 59-144）
  ├─ struct ConsoleBox (66-71)
  ├─ impl ConsoleBox (66-96)
  │  ├─ new()
  │  ├─ log()
  │  ├─ println()
  │  ├─ warn()
  │  ├─ error()
  │  └─ clear()
  ├─ impl BoxCore (100-120)
  └─ impl NyashBox (123-144)

📦 非WASM版（行 147-229）  ← 全く同じ構造！
  ├─ struct ConsoleBox (148-151)
  ├─ impl ConsoleBox (154-182)
  │  ├─ new()
  │  ├─ log()
  │  ├─ println()
  │  ├─ warn()
  │  ├─ error()
  │  └─ clear()
  ├─ impl BoxCore (185-205)
  └─ impl NyashBox (208-229)

📦 Display（行 232-244）
  ├─ WASM版 Display impl (233-237)
  └─ 非WASM版 Display impl (240-244)
```

### 重複の詳細

#### 1. **メソッド実装の分岐**（log, warn, error, clear）

```rust
// WASM版
pub fn log(&self, message: &str) {
    web_sys::console::log_1(&message.into());
}

// 非WASM版
pub fn log(&self, message: &str) {
    println!("[Console LOG] {}", message);
}
```

**問題**: 実装は異なるが、構造は完全に同じ

#### 2. **println の重複実装**

両方のバージョンで同じ実装:
```rust
pub fn println(&self, message: &str) {
    self.log(message);  // 常に log に委譲
}
```

**改善機会**: マクロで1度だけ定義

#### 3. **BoxCore の完全な重複**

```rust
// WASM版 impl BoxCore (100-120)
fn box_id(&self) -> u64 { self.base.id }
fn parent_type_id(&self) -> Option<std::any::TypeId> { self.base.parent_type_id }
fn fmt_box(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "[ConsoleBox - Browser Console Interface]")
}
// ... as_any, as_any_mut

// 非WASM版 impl BoxCore (185-205)
fn box_id(&self) -> u64 { self.base.id }
fn parent_type_id(&self) -> Option<std::any::TypeId> { self.base.parent_type_id }
fn fmt_box(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "[ConsoleBox - Mock Implementation]")  // ← ここだけ異なる！
}
// ... as_any, as_any_mut
```

**改善機会**: `fmt_box` のメッセージのみを条件付きにすれば共通化できる

#### 4. **NyashBox の完全な重複**

```rust
// 両バージョンで完全に同じ
impl NyashBox for ConsoleBox {
    fn to_string_box(&self) -> StringBox { ... }
    fn equals(&self, other: &dyn NyashBox) -> BoolBox { ... }
    fn type_name(&self) -> &'static str { "ConsoleBox" }
    fn clone_box(&self) -> Box<dyn NyashBox> { ... }
    fn share_box(&self) -> Box<dyn NyashBox> { ... }
}
```

**改善機会**: 完全に共通化可能

#### 5. **Display の重複**

```rust
// WASM版
impl Display for ConsoleBox {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.fmt_box(f)
    }
}

// 非WASM版 - 完全に同じ
impl Display for ConsoleBox {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.fmt_box(f)
    }
}
```

**改善機会**: 両バージョンを削除して1つに統合

## 実装戦略

### 戦略 A: マクロベースの統一（推奨）

マクロ `macro_rules! define_console_methods!` を使用してメソッド実装を統一化

```rust
// マクロ定義
macro_rules! define_console_methods {
    (
        log: $log_impl:expr,
        warn: $warn_impl:expr,
        error: $error_impl:expr,
        clear: $clear_impl:expr
    ) => {
        impl ConsoleBox {
            pub fn new() -> Self {
                Self {
                    base: BoxBase::new(),
                }
            }

            pub fn log(&self, message: &str) {
                $log_impl(self, message);
            }

            pub fn println(&self, message: &str) {
                self.log(message);  // 共通実装
            }

            pub fn warn(&self, message: &str) {
                $warn_impl(self, message);
            }

            pub fn error(&self, message: &str) {
                $error_impl(self, message);
            }

            pub fn clear(&self) {
                $clear_impl(self);
            }
        }
    };
}

// WASM環境での使用
#[cfg(target_arch = "wasm32")]
define_console_methods!(
    log: |s: &ConsoleBox, msg: &str| { web_sys::console::log_1(&msg.into()); },
    warn: |s: &ConsoleBox, msg: &str| { web_sys::console::warn_1(&msg.into()); },
    error: |s: &ConsoleBox, msg: &str| { web_sys::console::error_1(&msg.into()); },
    clear: |s: &ConsoleBox| { web_sys::console::clear(); }
);

// 非WASM環境での使用
#[cfg(not(target_arch = "wasm32"))]
define_console_methods!(
    log: |s: &ConsoleBox, msg: &str| { println!("[Console LOG] {}", msg); },
    warn: |s: &ConsoleBox, msg: &str| { println!("[Console WARN] {}", msg); },
    error: |s: &ConsoleBox, msg: &str| { println!("[Console ERROR] {}", msg); },
    clear: |s: &ConsoleBox| { println!("[Console CLEAR]"); }
);
```

### 戦略 B: trait オブジェクトによる実装（代替案）

```rust
trait ConsolePlatform {
    fn log(&self, message: &str);
    fn warn(&self, message: &str);
    fn error(&self, message: &str);
    fn clear(&self);
}

// WASM版
#[cfg(target_arch = "wasm32")]
struct WasmPlatform;

#[cfg(target_arch = "wasm32")]
impl ConsolePlatform for WasmPlatform {
    // WASM実装
}

// 非WASM版
#[cfg(not(target_arch = "wasm32"))]
struct MockPlatform;

// ConsoleBox はプラットフォームを集約
struct ConsoleBox {
    base: BoxBase,
    platform: Box<dyn ConsolePlatform>,  // ← 委譲
}
```

**注記**: 本フェーズではマクロベースを推奨（シンプルで効果的）

## 実装ステップ

### ステップ 1: マクロ定義の作成

新しいマクロを定義:

```rust
// 行 54 から 57 の use 宣言直後に挿入

/// ConsoleBox メソッド実装マクロ
/// WASM/非WASM環境で異なるメソッド実装を統一化
macro_rules! define_console_impl {
    (
        log: $log_impl:expr,
        warn: $warn_impl:expr,
        error: $error_impl:expr,
        clear: $clear_impl:expr,
        fmt_desc: $fmt_desc:expr
    ) => {
        impl ConsoleBox {
            pub fn new() -> Self {
                Self {
                    base: BoxBase::new(),
                }
            }

            pub fn log(&self, message: &str) {
                $log_impl(message);
            }

            pub fn println(&self, message: &str) {
                self.log(message);
            }

            pub fn warn(&self, message: &str) {
                $warn_impl(message);
            }

            pub fn error(&self, message: &str) {
                $error_impl(message);
            }

            pub fn clear(&self) {
                $clear_impl();
            }
        }

        impl BoxCore for ConsoleBox {
            fn box_id(&self) -> u64 {
                self.base.id
            }

            fn parent_type_id(&self) -> Option<std::any::TypeId> {
                self.base.parent_type_id
            }

            fn fmt_box(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{}", $fmt_desc)
            }

            fn as_any(&self) -> &dyn Any {
                self
            }

            fn as_any_mut(&mut self) -> &mut dyn Any {
                self
            }
        }

        impl NyashBox for ConsoleBox {
            fn to_string_box(&self) -> StringBox {
                StringBox::new($fmt_desc)
            }

            fn equals(&self, other: &dyn NyashBox) -> BoolBox {
                BoolBox::new(other.as_any().is::<ConsoleBox>())
            }

            fn type_name(&self) -> &'static str {
                "ConsoleBox"
            }

            fn clone_box(&self) -> Box<dyn NyashBox> {
                Box::new(self.clone())
            }

            fn share_box(&self) -> Box<dyn NyashBox> {
                self.clone_box()
            }
        }

        impl Display for ConsoleBox {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.fmt_box(f)
            }
        }
    };
}
```

### ステップ 2: WASM版の実装

```rust
#[cfg(target_arch = "wasm32")]
#[derive(Debug, Clone)]
pub struct ConsoleBox {
    base: BoxBase,
}

#[cfg(target_arch = "wasm32")]
define_console_impl!(
    log: |msg: &str| { web_sys::console::log_1(&msg.into()); },
    warn: |msg: &str| { web_sys::console::warn_1(&msg.into()); },
    error: |msg: &str| { web_sys::console::error_1(&msg.into()); },
    clear: || { web_sys::console::clear(); },
    fmt_desc: "[ConsoleBox - Browser Console Interface]"
);
```

### ステップ 3: 非WASM版の実装

```rust
#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug, Clone)]
pub struct ConsoleBox {
    base: BoxBase,
}

#[cfg(not(target_arch = "wasm32"))]
define_console_impl!(
    log: |msg: &str| { println!("[Console LOG] {}", msg); },
    warn: |msg: &str| { println!("[Console WARN] {}", msg); },
    error: |msg: &str| { println!("[Console ERROR] {}", msg); },
    clear: || { println!("[Console CLEAR]"); },
    fmt_desc: "[ConsoleBox - Mock Implementation]"
);
```

### ステップ 4: 検証

```bash
# コンパイル確認
cargo build --release 2>&1 | grep -i console

# 機能確認
NYASH_JOINIR_STRICT=1 ./target/release/nyash apps/tests/peek_expr_block.hako
NYASH_JOINIR_STRICT=1 ./target/release/nyash apps/tests/loop_min_while.hako
NYASH_JOINIR_STRICT=1 ./target/release/nyash apps/tests/esc_dirname_smoke.hako

# WASM ビルド確認（もし WASM 環境が整っていれば）
cargo build --release --target wasm32-unknown-unknown 2>&1 | grep -i console
```

## 期待される効果

### コード削減
- **削減行数**: ~65行（25%削減）
  - WASM版の重複実装: ~30行削除
  - 非WASM版の重複実装: ~30行削除
  - Display 重複: ~10行削除
  - マクロ定義による新規追加: ~15行

- **最終サイズ**: 245行 → 180行程度

### 保守性向上
- メソッド実装を1箇所で管理
- WASM/非WASM の差分が明確
- println の一元化実装
- BoxCore/NyashBox の重複削除

### 可読性向上
- マクロの引数で環境別の違いが明示的
- println が log に委譲される流れが可視化
- fmt_desc で環境別の説明文が分離

## 実装上の注意

### 1. マクロの引数形式

```rust
// ✅ 正しい
define_console_impl!(
    log: |msg: &str| { /* 実装 */ },
    // ...
);

// ❌ 避けるべき
define_console_impl!(
    log = |msg: &str| { /* 実装 */ },  // = は使わない
);
```

### 2. println のクロージャ

```rust
// ❌ self が必要ない場合でも self を渡す
println: |msg: &str| { self.log(msg); }  // self は available ではない

// ✅ 正しい（log に委譲は ConsoleBox メソッド内で実装）
// マクロ内で `pub fn println(&self, message: &str) { self.log(message); }` を生成
```

### 3. Display 統合

現在の実装:
```rust
#[cfg(target_arch = "wasm32")]
impl Display for ConsoleBox { ... }

#[cfg(not(target_arch = "wasm32"))]
impl Display for ConsoleBox { ... }
```

マクロ後は両 cfg ブロック内で定義されるため、`impl Display` は削除可能。

## ロールバック計画

修正後に問題が発生した場合：

```bash
# 修正前のバージョンに戻す
git checkout HEAD~ -- src/boxes/console_box.rs

# または具体的な commit hash を指定
git show <commit-hash>:src/boxes/console_box.rs > src/boxes/console_box.rs
```

## テスト戦略

### ユニットテスト（新規作成不要）

現在、ConsoleBox は直接テストしていない（プラグイン経由）。

### 統合テスト

既存の Phase 120 representative tests が十分：

```bash
# すべてが成功することを確認
NYASH_JOINIR_STRICT=1 ./target/release/nyash apps/tests/esc_dirname_smoke.hako
# 出力: [Console LOG] dir1/dir2
```

## 所要時間

**2時間程度**

- マクロ設計と定義: 30分
- WASM/非WASM 実装: 45分
- 検証とテスト: 30分
- ドキュメント更新: 15分

## 完了後の次のステップ

Phase 124: VM Method Dispatch 統一化（4時間）

---

**記録**:
- Phase 122.5: nyash.toml method_id 修正 ✅ 完了
- Phase 123: ConsoleBox WASM/非WASM 統一化 ← **現在のフェーズ**
- Phase 124: VM Method Dispatch 統一化（予定）
- Phase 125: 削除：deprecated builtin ConsoleBox（予定）
- Phase 126: ドキュメント統合（予定）
Status: Historical
