# ConsoleBox Complete Guide - デザイン・実装・運用

## 📖 目次

1. [概要・歴史](#1-概要歴史)
2. [ユーザーガイド](#2-ユーザーガイド)
3. [アーキテクチャ設計](#3-アーキテクチャ設計)
4. [実装者向けガイド](#4-実装者向けガイド)
5. [FAQ・トラブルシューティング](#5-faqトラブルシューティング)

---

## 1. 概要・歴史

### ConsoleBox の進化（Phase 122-125）

ConsoleBox は Phase 122-125 にわたる一連の改善を経て、現在の形に進化しました。各フェーズで特定の問題を解決し、アーキテクチャの洗練化を図りました。

#### Phase 122: println/log エイリアス統一

**背景**:
- `apps/tests/esc_dirname_smoke.hako` が selfhost Stage-3 + JoinIR Strict 経路で失敗
- エラーメッセージ: `Unknown method 'println' on ConsoleBox`
- 原因: `.hako` サンプルは `console.println()` を使用するが、Rust 実装は `log()` のみ実装

**実装内容**:
- `println` を `log` のエイリアスとして定義
- TypeRegistry で slot 400（log と同じ）に統一
- すべての経路（JSON v0 / selfhost / 通常VM）で一貫性を保証

**技術的詳細**:
```rust
// src/runtime/type_registry.rs
const CONSOLE_METHODS: &[MethodEntry] = &[
    MethodEntry { name: "log",     arity: 1, slot: 400 },
    MethodEntry { name: "warn",    arity: 1, slot: 401 },
    MethodEntry { name: "error",   arity: 1, slot: 402 },
    MethodEntry { name: "clear",   arity: 0, slot: 403 },
    // Phase 122: println は log のエイリアス
    MethodEntry { name: "println", arity: 1, slot: 400 },
];
```

**成果**:
- ユーザー向けAPI（`println`）と VM実装（`log`）の統一
- selfhost Stage-3 経路での完全動作
- 正規化ポイントを TypeRegistry に一元化

**参照**: [Phase 122 詳細ドキュメント](phase122_consolebox_println_unification.md)

---

#### Phase 122.5: nyash.toml method_id 修正

**背景**:
- Phase 122 で println/log の統一が完了したが、nyash.toml の method_id が不整合
- ConsoleBox プラグインの method_id が slot 400 と異なる値（1）になっていた

**実装内容**:
- nyash.toml の `[libraries."libnyash_console_plugin.so".ConsoleBox.methods]` で method_id を統一
- `log` と `println` の両方を method_id = 400 に設定

**技術的詳細**:
```toml
# nyash.toml（修正後）
[libraries."libnyash_console_plugin.so".ConsoleBox.methods]
log = { method_id = 400 }
println = { method_id = 400 }  # log と同じ
warn = { method_id = 401 }
error = { method_id = 402 }
clear = { method_id = 403 }
```

**成果**:
- TypeRegistry と nyash.toml の完全な一致
- プラグインと VM 実装の完全な統一

**参照**: [Phase 122.5 詳細ドキュメント](phase122_5_nyash_toml_fix.md)

---

#### Phase 123: WASM/非WASM コード統一

**背景**:
- `src/boxes/console_box.rs` で WASM/非WASM 環境の実装が完全に重複
- ~245行の実装に ~85行の重複が存在（35%が重複）

**重複内容**:
1. メソッド実装の分岐（log, warn, error, clear）
2. println の重複実装（両バージョンで同一）
3. BoxCore の完全な重複（fmt_box のメッセージのみ異なる）
4. NyashBox の完全な重複（完全に同一）
5. Display の重複（完全に同一）

**実装内容**:
- マクロ `define_console_impl!` を使用してメソッド実装を統一化
- WASM版と非WASM版でクロージャ実装のみを差分化
- BoxCore/NyashBox/Display を1つのマクロ内で生成

**技術的詳細**:
```rust
// マクロ定義
macro_rules! define_console_impl {
    (
        log: $log_impl:expr,
        warn: $warn_impl:expr,
        error: $error_impl:expr,
        clear: $clear_impl:expr,
        fmt_desc: $fmt_desc:expr
    ) => {
        // impl ConsoleBox
        // impl BoxCore
        // impl NyashBox
        // impl Display
        // すべてをマクロ内で生成
    };
}

// WASM環境での使用
#[cfg(target_arch = "wasm32")]
define_console_impl!(
    log: |msg: &str| { web_sys::console::log_1(&msg.into()); },
    warn: |msg: &str| { web_sys::console::warn_1(&msg.into()); },
    error: |msg: &str| { web_sys::console::error_1(&msg.into()); },
    clear: || { web_sys::console::clear(); },
    fmt_desc: "[ConsoleBox - Browser Console Interface]"
);

// 非WASM環境での使用
#[cfg(not(target_arch = "wasm32"))]
define_console_impl!(
    log: |msg: &str| { println!("[Console LOG] {}", msg); },
    warn: |msg: &str| { println!("[Console WARN] {}", msg); },
    error: |msg: &str| { println!("[Console ERROR] {}", msg); },
    clear: || { println!("[Console CLEAR]"); },
    fmt_desc: "[ConsoleBox - Mock Implementation]"
);
```

**成果**:
- ~67行削減（27.3%削減）
- 245行 → 178行に削減
- WASM/非WASM の差分が明確
- println の一元化実装

**参照**: [Phase 123 詳細ドキュメント](phase123_consolebox_code_unification.md)

---

#### Phase 124: TypeRegistry ベースの統一ディスパッチ

**背景**:
- `src/backend/mir_interpreter/handlers/calls/method.rs` の `execute_method_call()` が型ごとの手動 match ベース
- String, Array, ConsoleBox などで異なる処理パス
- ビルトイン型とプラグイン Box のメソッド解決が統一されていない

**問題点**:
1. 統一性の欠如: String のメソッドと BoxRef のメソッドで異なる処理パス
2. スケーラビリティの悪さ: 新しい型を追加するたびに新しい match arm が必要
3. 二重実装の危険性: StringBox と String の両方にメソッド実装がある可能性
4. 保守性: TypeRegistry と VM 実装が乖離

**実装内容**:
- ビルトイン型（String, Integer, Array）を TypeRegistry に登録
- 統一ディスパッチ関数 `dispatch_by_slot()` を実装
- execute_method_call を簡略化（type_id → slot → dispatch）

**技術的詳細**:
```rust
// 統一ディスパッチのフロー
fn execute_method_call(
    &mut self,
    receiver: &VMValue,
    method: &str,
    args: &[ValueId],
) -> Result<VMValue, VMError> {
    // 1. receiver から type_name を取得
    let type_name = match receiver {
        VMValue::String(_) => "String",
        VMValue::BoxRef(bx) => bx.type_name(),
        // ...
    };

    // 2. TypeRegistry で type_id を検索
    let type_id = self.type_registry.lookup_type_id(type_name)?;

    // 3. MethodEntry から slot を取得
    let slot = self.type_registry
        .lookup_method_slot(type_id, method)?;

    // 4. 統一ディスパッチ
    self.dispatch_by_slot(receiver, type_id, slot, args)
}
```

**成果**:
- ~100行削減（method.rs の簡略化）
- String, Array, ConsoleBox を統一的に dispatch_by_slot で処理
- メソッド解決の一元化（TypeRegistry）
- スロット番号による高速ディスパッチ

**参照**: [Phase 124 詳細ドキュメント](phase124_vm_method_dispatch_unification.md)

---

#### Phase 125: ビルトイン ConsoleBox 削除

**背景**:
- ビルトイン ConsoleBox（`src/box_factory/builtin_impls/console_box.rs`）とプラグイン ConsoleBox の二重実装
- "Everything is Plugin" 原則の実装完成へ

**削除対象**:
1. `src/box_factory/builtin_impls/console_box.rs`（36行）
2. `src/box_factory/builtin.rs` の ConsoleBox case（~10行）
3. `src/box_factory/builtin_impls/mod.rs` の mod 宣言（~2行）
4. テストコード（~10行）

**重要な注意**:
- `src/boxes/console_box.rs` は削除しない！
- これは Rust 実装（VM が内部的に使用）
- プラグイン（libnyash_console_plugin.so）が内部で使用している
- Phase 124 で TypeRegistry ベースの dispatch_by_slot に統合済み

**実装内容**:
```rust
// builtin.rs の修正（削除）
match name {
    // Phase 125: ✅ DELETED - ConsoleBox is now plugin-only!
    // See: plugins/nyash-console-plugin for current implementation

    // ... 他のビルトイン Box

    _ => Err(RuntimeError::InvalidOperation {
        message: format!("Unknown Box type: {}", name),
    }),
}
```

**成果**:
- ~52行削減
- ビルトイン Factory の簡略化
- "Everything is Plugin" 原則の完全実装
- プラグインのみへの移行完了

**参照**: [Phase 125 詳細ドキュメント](phase125_delete_deprecated_console_box.md)

---

### 現在の状態（Phase 126 以降）

✅ ConsoleBox はプラグインのみ
✅ Rust 実装（`src/boxes/console_box.rs`）は内部用
✅ TypeRegistry ベースのディスパッチ
✅ println/log 統一
✅ WASM/非WASM 統一
✅ ビルトイン ConsoleBox 削除完了

**総削減コード量**: ~219行削減
- Phase 122: TypeRegistry 修正（+数行、正規化）
- Phase 122.5: nyash.toml 修正（+数行）
- Phase 123: コード統一化（67行削減）
- Phase 124: VM Method Dispatch 統一化（100行削減）
- Phase 125: ビルトイン ConsoleBox 削除（52行削減）

---

## 2. ユーザーガイド

### 基本的な使用方法

```nyash
// ConsoleBox インスタンス作成
local console
console = new ConsoleBox()

// 通常ログ（推奨）
console.println("Hello, Nyash!")
console.log("Same as println")

// 警告・エラー
console.warn("This is a warning")
console.error("Something went wrong")

// 画面クリア
console.clear()
```

### println vs log

Phase 122 以降、`println` と `log` は完全に同じです（`println` = `log` のエイリアス）。

**推奨**: ユーザーコードでは `println` を使用（ユーザー向け API）

**理由**:
- `println` は他のプログラミング言語（JavaScript, Python, Java など）との一貫性
- `log` は内部実装用（互換性のみのため）

### WASM 環境での動作

ブラウザの開発者ツール（F12）のコンソールに出力されます。

**例**:
```nyash
local console = new ConsoleBox()
console.println("Hello from WASM!")
// → ブラウザコンソールに "Hello from WASM!" が表示
```

**技術詳細**:
- WASM環境では `web_sys::console::log_1()` を使用
- ブラウザの Console API に直接出力
- 開発者ツールのフィルタリング・検索が使用可能

### ネイティブ環境での動作

標準出力にプレフィックス付きで出力されます。

**例**:
```nyash
local console = new ConsoleBox()
console.println("Hello, Nyash!")
console.warn("This is a warning")
console.error("Something went wrong")
console.clear()
```

**出力**:
```
[Console LOG] Hello, Nyash!
[Console WARN] This is a warning
[Console ERROR] Something went wrong
[Console CLEAR]
```

### 静的 Box での使用

```nyash
static box Main {
    console: ConsoleBox

    main() {
        me.console = new ConsoleBox()
        me.console.println("Application started")

        // ... 処理

        me.console.println("Application finished")
    }
}
```

### エラー出力のベストプラクティス

```nyash
box FileProcessor {
    console: ConsoleBox

    process(filename) {
        me.console = new ConsoleBox()

        if file_not_found(filename) {
            me.console.error("❌ File not found: " + filename)
            return -1
        }

        me.console.println("✅ Processing: " + filename)
        return 0
    }
}
```

---

## 3. アーキテクチャ設計

### 3層ロギングアーキテクチャ

ConsoleBox は Nyash の3層ロギングシステムの中核を担います。

```
┌─────────────────────────────────────────────────────────┐
│  ユーザーアプリケーション (.hako)                        │
│                                                          │
│  box MyApp {                                            │
│    main() {                                             │
│      me.console.println("Result: OK")  ← ConsoleBox     │
│    }                                                    │
│  }                                                      │
└──────────────────┬──────────────────────────────────────┘
                   │
              ConsoleService
              (user-facing)
                   │
┌──────────────────▼──────────────────────────────────────┐
│  Rust Runtime (Ring0.log)                               │
│                                                          │
│  ring0.log.debug("[joinir] Processing...")      ← internal
│  ring0.log.error("VM error: {}")                ← internal
└──────────────────┬──────────────────────────────────────┘
                   │
                stderr/stdout
                   │
           ┌───────▼────────┐
           │  Terminal      │
           │  (user sees)   │
           └────────────────┘
```

**3つの層**:

1. **Ring0.log（OS APIレイヤー）**: Runtime/OS層内部ログ（開発者向け）
2. **ConsoleService（Boxレイヤー）**: ユーザー向けCLI出力（エンドユーザー向け）
3. **Raw println!/eprintln!**: テスト・デバッグ専用（本番では制限）

**参照**:
- [ログポリシー](logging_policy.md)
- [Hako ログ設計](hako_logging_design.md)

---

### TypeRegistry ベースのディスパッチ（Phase 124）

Phase 124 で実装された統一ディスパッチシステムにより、ConsoleBox のメソッド解決が完全に TypeRegistry に統合されました。

**メソッド解決フロー**:

```
ユーザーコード: console.println("Hello")
     ↓
VM: execute_method_call(receiver, "println", args)
     ↓
1. type_name = "ConsoleBox"
2. type_id = 7 (TypeRegistry lookup)
3. slot = 400 (method "println" → slot lookup)
     ↓
dispatch_by_slot(receiver, type_id=7, slot=400, args)
     ↓
ConsoleBox 実装: log の実装が実行される
     ↓
出力: [Console LOG] Hello
```

**利点**:
- ビルトイン型（String, Array）とプラグイン Box の統一処理
- スロット番号による高速ディスパッチ
- メソッド解決の一元化

**参照**: [Phase 124 詳細](phase124_vm_method_dispatch_unification.md)

---

### プラグインアーキテクチャ（Phase 125）

Phase 125 でビルトイン ConsoleBox を削除し、完全にプラグインベースに移行しました。

**現在の実装構造**:

```
src/boxes/console_box.rs        ← Rust 実装（VM が内部的に使用）
   ↓
libnyash_console_plugin.so      ← プラグイン（ユーザー向けインターフェース）
   ↓
src/box_factory/builtin.rs      ← ビルトイン Factory（ConsoleBox は削除済み）
```

**"Everything is Plugin" 原則**:
- すべての Box はプラグインベース（StringBox, IntegerBox, ArrayBox も移行予定）
- ビルトイン Factory の複雑性低減
- プラグイン拡張性の向上

**参照**:
- [Phase 125 詳細](phase125_delete_deprecated_console_box.md)
- [Core Boxes 設計](core_boxes_design.md)

---

### println/log エイリアス設計（Phase 122）

Phase 122 で確立された「Alias First」設計原則により、複数の名前を持つ API は VM レベルで alias に統一されます。

**正規化ポイント**: TypeRegistry

```rust
// src/runtime/type_registry.rs
const CONSOLE_METHODS: &[MethodEntry] = &[
    MethodEntry { name: "log",     arity: 1, slot: 400 },
    MethodEntry { name: "println", arity: 1, slot: 400 },  // log と同じ slot
    MethodEntry { name: "warn",    arity: 1, slot: 401 },
    MethodEntry { name: "error",   arity: 1, slot: 402 },
    MethodEntry { name: "clear",   arity: 0, slot: 403 },
];
```

**重要な約束**:
- alias は TypeRegistry で管理（VM レベルで一元管理）
- MirBuilder は関与しない（特別扱いなし）
- すべての経路で一貫（JSON v0 / selfhost / 通常VM）

**参照**: [Phase 122 詳細](phase122_consolebox_println_unification.md)

---

### WASM/非WASM 統一設計（Phase 123）

Phase 123 で実装されたマクロベースの統一設計により、WASM/非WASM 環境の実装重複を排除しました。

**マクロ設計**:

```rust
macro_rules! define_console_impl {
    (
        log: $log_impl:expr,
        warn: $warn_impl:expr,
        error: $error_impl:expr,
        clear: $clear_impl:expr,
        fmt_desc: $fmt_desc:expr
    ) => {
        // 統一実装（impl ConsoleBox, BoxCore, NyashBox, Display）
    };
}
```

**環境別の差分**:

| 環境 | log 実装 | fmt_desc |
|------|---------|----------|
| WASM | `web_sys::console::log_1(&msg.into())` | "Browser Console Interface" |
| 非WASM | `println!("[Console LOG] {}", msg)` | "Mock Implementation" |

**利点**:
- 重複削減（67行削減、27.3%）
- 環境別の差分が明確
- 保守性向上（1箇所修正で両環境に反映）

**参照**: [Phase 123 詳細](phase123_consolebox_code_unification.md)

---

## 4. 実装者向けガイド

### TypeRegistry での slot 管理

ConsoleBox のメソッドは TypeRegistry で以下のスロットに割り当てられています。

**slot 定義**:

| メソッド | arity | slot | 説明 |
|---------|-------|------|------|
| `log` | 1 | 400 | コアメソッド（標準出力） |
| `println` | 1 | 400 | log のエイリアス |
| `warn` | 1 | 401 | 警告メッセージ |
| `error` | 1 | 402 | エラーメッセージ |
| `clear` | 0 | 403 | コンソールクリア |

**type_id**: ConsoleBox の type_id は 7

**参照**: `src/runtime/type_registry.rs`

---

### VM Method Dispatch の仕組み（Phase 124）

Phase 124 で実装された統一ディスパッチシステムにより、ConsoleBox のメソッド呼び出しは以下のフローで処理されます。

**ディスパッチフロー**:

1. **execute_method_call()** が呼ばれる
2. receiver から type_name を取得（"ConsoleBox"）
3. TypeRegistry で type_id を検索（7）
4. TypeRegistry で method_name から slot を検索（例: "println" → 400）
5. **dispatch_by_slot()** でスロットベースのディスパッチ
6. ConsoleBox 実装が実行される

**コード例**:

```rust
// src/backend/mir_interpreter/handlers/calls/method.rs
fn execute_method_call(
    &mut self,
    receiver: &VMValue,
    method: &str,
    args: &[ValueId],
) -> Result<VMValue, VMError> {
    let type_name = match receiver {
        VMValue::BoxRef(bx) => bx.type_name(),
        // ...
    };

    let type_id = self.type_registry.lookup_type_id(type_name)?;
    let slot = self.type_registry.lookup_method_slot(type_id, method)?;

    self.dispatch_by_slot(receiver, type_id, slot, args)
}
```

---

### プラグイン ConsoleBox の拡張方法

ConsoleBox はプラグインベースのため、拡張が容易です。

**拡張手順**:

1. **Rust 実装の拡張**（`src/boxes/console_box.rs`）:
```rust
impl ConsoleBox {
    pub fn new_method(&self, arg: &str) {
        // 新しいメソッドの実装
    }
}
```

2. **TypeRegistry へのメソッド追加**（`src/runtime/type_registry.rs`）:
```rust
const CONSOLE_METHODS: &[MethodEntry] = &[
    // ... 既存のメソッド
    MethodEntry { name: "new_method", arity: 1, slot: 404 },
];
```

3. **nyash.toml へのメソッド登録**:
```toml
[libraries."libnyash_console_plugin.so".ConsoleBox.methods]
new_method = { method_id = 404 }
```

4. **プラグインビルド**:
```bash
cargo build --release -p nyash-console-plugin
```

**注意点**:
- slot 番号は既存のメソッドと重複しないように
- TypeRegistry と nyash.toml の method_id を一致させる
- WASM版の実装も忘れずに（必要な場合）

---

### デバッグとトレース

ConsoleBox の動作をデバッグする際の環境変数と手法：

**環境変数**:

```bash
# VM 詳細診断
NYASH_CLI_VERBOSE=1 ./target/release/nyash program.hako

# MIR 出力（メソッド呼び出し確認）
NYASH_VM_DUMP_MIR=1 ./target/release/nyash program.hako

# TypeRegistry デバッグ
NYASH_DEBUG_TYPE_REGISTRY=1 ./target/release/nyash program.hako
```

**MIR 出力例**:

```
Function: main
Block 0:
  r1 = NewBox("ConsoleBox")
  r2 = Const(String("Hello"))
  r3 = BoxCall(r1, "println", [r2])  // ← println メソッド呼び出し
  Return(r3)
```

**TypeRegistry lookup のトレース**:

```
[TypeRegistry] lookup_type_id("ConsoleBox") → 7
[TypeRegistry] lookup_method_slot(7, "println") → 400
[VM] dispatch_by_slot(type_id=7, slot=400)
[Console LOG] Hello
```

---

### テスト戦略

ConsoleBox のテストは以下の3つのレベルで実施します。

**1. ユニットテスト**（Rust レベル）:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_console_box_creation() {
        let console = ConsoleBox::new();
        assert_eq!(console.type_name(), "ConsoleBox");
    }

    #[test]
    fn test_println_is_log_alias() {
        let console = ConsoleBox::new();
        // println と log が同じ動作をすることを確認
        console.println("test");  // OK
        console.log("test");      // OK
    }
}
```

**2. 統合テスト**（.hako レベル）:

```bash
# Phase 120 representative tests
NYASH_JOINIR_STRICT=1 ./target/release/nyash apps/tests/esc_dirname_smoke.hako
# 出力: [Console LOG] dir1/dir2

# println テスト
echo 'local c = new ConsoleBox(); c.println("Hello")' > test.hako
./target/release/nyash test.hako
# 出力: [Console LOG] Hello
```

**3. スモークテスト**:

```bash
tools/smokes/v2/run.sh --profile quick
```

---

## 5. FAQ・トラブルシューティング

### Q: println と log の違いは？

**A**: Phase 122 以降、完全に同じです（`println` = `log` のエイリアス）。

**詳細**:
- VM の TypeRegistry で両者は同じ slot 400 を使用
- 内部実装は完全に同一
- ユーザーコードでは `println` を使用することを推奨

**参照**: [Phase 122 詳細](phase122_consolebox_println_unification.md)

---

### Q: println が動作しない

**A**: プラグイン（`libnyash_console_plugin.so`）が読み込まれているか確認してください。

**確認方法**:

```bash
# プラグインファイルの存在確認
ls target/release/libnyash_console_plugin.so

# nyash.toml の設定確認
rg "ConsoleBox" nyash.toml

# プラグインなしで実行してエラーメッセージ確認
NYASH_DISABLE_PLUGINS=1 ./target/release/nyash test.hako
```

**エラーメッセージ例**:

```
Error: Unknown Box type: ConsoleBox
```

**解決方法**:
- プラグインをビルド: `cargo build --release -p nyash-console-plugin`
- nyash.toml にプラグイン設定を追加

---

### Q: println と log を区別する必要があります

**A**: TypeRegistry の slot をカスタマイズして異なる slot を割り当てることは可能ですが、推奨されません。

**理由**:
- Phase 122 の設計原則に反する
- 全経路（JSON v0 / selfhost / 通常VM）での一貫性が失われる
- 保守性が低下する

**代替案**:
- 用途に応じて `warn()`, `error()` を使用
- カスタムメソッドを追加（例: `debug()`, `trace()`）

---

### Q: WASM 環境でコンソール出力が見えない

**A**: ブラウザの開発者ツール（F12）のコンソールを開いてください。

**確認手順**:

1. ブラウザで F12 キーを押す
2. "Console" タブを選択
3. `.hako` アプリケーションを実行
4. コンソールに出力が表示される

**トラブルシューティング**:
- ブラウザのコンソールフィルタが有効になっていないか確認
- JavaScript エラーでコンソール出力がブロックされていないか確認

---

### Q: 非WASM 環境で出力にプレフィックスが付く

**A**: これは Phase 123 の設計です。非WASM 環境では `[Console LOG]` などのプレフィックスが付きます。

**理由**:
- デバッグ・トレースの容易性
- 他の出力（Rust runtime のログなど）との区別

**出力例**:

```
[Console LOG] Hello
[Console WARN] Warning message
[Console ERROR] Error occurred
[Console CLEAR]
```

**プレフィックスを削除したい場合**:
- WASM 環境を使用（ブラウザコンソールに直接出力）
- カスタムプラグインを実装（プレフィックスなし版）

---

### Q: ConsoleBox のメソッドを追加したい

**A**: [実装者向けガイド - プラグイン ConsoleBox の拡張方法](#プラグイン-consolebox-の拡張方法)を参照してください。

**概要**:
1. Rust 実装の拡張
2. TypeRegistry へのメソッド追加
3. nyash.toml への登録
4. プラグインビルド

---

### Q: Phase 122-125 のドキュメントはどこにありますか？

**A**: 各フェーズの詳細実装記録は以下のファイルを参照してください。

**Phase 122-125 実装記録**:
- [Phase 122: println/log 統一](phase122_consolebox_println_unification.md)
- [Phase 122.5: nyash.toml 修正](phase122_5_nyash_toml_fix.md)
- [Phase 123: WASM/非WASM 統一](phase123_consolebox_code_unification.md)
- [Phase 124: VM Method Dispatch 統一](phase124_vm_method_dispatch_unification.md)
- [Phase 125: ビルトイン ConsoleBox 削除](phase125_delete_deprecated_console_box.md)

**関連ドキュメント**:
- [Core Boxes 設計](core_boxes_design.md)
- [ログポリシー](logging_policy.md)
- [Hako ログ設計](hako_logging_design.md)

---

### Q: 他の Box（StringBox, ArrayBox など）も同じように統一されていますか？

**A**: Phase 124 で String, Array, ConsoleBox のメソッドディスパッチが TypeRegistry ベースで統一されました。

**統一内容**:
- ビルトイン型（String, Integer, Array）の TypeRegistry 登録
- 統一ディスパッチ関数 `dispatch_by_slot()`
- execute_method_call の簡略化

**今後の計画**:
- StringBox, IntegerBox, ArrayBox もプラグインベースに移行（Phase 15.5 計画）
- すべての Box が "Everything is Plugin" 原則に従う

**参照**:
- [Phase 124 詳細](phase124_vm_method_dispatch_unification.md)
- [Core Boxes 設計](core_boxes_design.md)

---

## 📚 Related Documents

### ConsoleBox について知りたい場合
- **このドキュメント**: 統合的なリファレンス
- [Phase 122-125 実装記録](phase122_consolebox_println_unification.md): 詳細な実装背景

### ログ出力について知りたい場合
- [ログポリシー](logging_policy.md): Nyash のログ出力全体のポリシー
- [Hako ログ設計](hako_logging_design.md): Hako コンパイラ側のログ設計

### アーキテクチャについて知りたい場合
- [Core Boxes 設計](core_boxes_design.md): Core Box の全体設計
- [TypeRegistry 設計](../architecture/type-registry-design.md): TypeRegistry の詳細設計

---

## 📅 Document Version

- **Last Updated**: Phase 126 (2025-12-04)
- **Scope**: ConsoleBox API, Architecture, Implementation
- **Applies to**: Nyash Release (Phase 122-125 完了後)

---

**Phase 126 統合ドキュメント完成日**: 2025-12-04
