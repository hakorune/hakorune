# Phase 125: 削除：deprecated builtin ConsoleBox

⚠️ **Note**: このドキュメントは Phase 125 の実装記録です。
           統合的なガイドは [ConsoleBox 完全ガイド](consolebox_complete_guide.md) をご参照ください。

## 目的

ビルトイン ConsoleBox 実装を完全に削除し、**プラグインベース（nyash-console-plugin）の ConsoleBox のみ** に移行する。

これにより、「Everything is Plugin」の原則を完全に実装し、Core Box Factory の複雑性を低減する。

## 背景

### 現在の二重実装

**Builtin ConsoleBox**（src/box_factory/builtin_impls/console_box.rs）:
- ビルトイン実装：36行
- 削除予定（Phase 125 = 今）

**Plugin ConsoleBox**（plugins/nyash-console-plugin/）:
- プラグイン実装：完全に動作
- Phase 122 で println のサポートを追加
- 推奨（使用すべき）

### 削除理由

1. **二重性の排除**: 同じ Box が2つの経路で実装されている
2. **Plugin-First 原則**: 全 Box をプラグイン化（Phase 15.5 目標）
3. **コード削減**: 不要なビルトイン実装削除
4. **保守性向上**: Core Factory の複雑性低減
5. **整合性確保**: TypeRegistry と Plugin System の統一（Phase 124 完了後）

## 削除対象

### 1. ファイル削除

**ファイル**: `src/box_factory/builtin_impls/console_box.rs` (36行)

```rust
// ❌ 削除対象
pub fn create(_args: &[Box<dyn NyashBox>]) -> Result<Box<dyn NyashBox>, RuntimeError> {
    eprintln!(
        "⚠️ [DEPRECATED] Using builtin ConsoleBox - use nyash-console-plugin!\n\
        📋 Phase 15.5: Everything is Plugin!\n\
        🔧 Check: plugins/nyash-console-plugin\n\
        ⚠️ WARNING: ConsoleBox is critical for logging - remove LAST!"
    );

    Ok(Box::new(crate::boxes::console_box::ConsoleBox::new()))
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_builtin_console_box_creation() {
        let result = create(&[]).unwrap();
        assert!(result.as_any().downcast_ref::<ConsoleBox>().is_some());
    }
}
```

**削除理由**: プラグインのみで十分動作

### 2. builtin.rs の修正

**ファイル**: `src/box_factory/builtin.rs`

#### 修正内容 1: create_box メソッドから ConsoleBox case を削除

現在（行 52）:
```rust
// Phase 2.6: DELETE LAST (critical for logging)
"ConsoleBox" => builtin_impls::console_box::create(args),
```

修正後:
```rust
// Phase 125: ✅ DELETED - Now plugin-only (nyash-console-plugin)
// ConsoleBox route deleted - must use plugin
```

#### 修正内容 2: box_types メソッドから ConsoleBox を削除

現在（行 79）:
```rust
vec![
    // Primitive wrappers
    "StringBox",
    "IntegerBox",
    "BoolBox",
    // Collections/common
    "ArrayBox",
    "MapBox",
    "ConsoleBox",  // ← 削除
    // Fallback support
    "FileBox",
    "FileHandleBox", // Phase 113
    "NullBox",
]
```

修正後:
```rust
vec![
    // Primitive wrappers
    "StringBox",
    "IntegerBox",
    "BoolBox",
    // Collections/common
    "ArrayBox",
    "MapBox",
    // ConsoleBox: Phase 125 - Plugin-only (nyash-console-plugin)
    // Fallback support
    "FileBox",
    "FileHandleBox", // Phase 113
    "NullBox",
]
```

### 3. builtin_impls/mod.rs の修正

**ファイル**: `src/box_factory/builtin_impls/mod.rs`

現在の状態:
```rust
//! Individual builtin Box implementations (easy deletion for Plugin migration)
//! ...
//! 6. console_box.rs   - Phase 2.6 🔄 Plugin exists, remove LAST

pub mod console_box;  // ← 削除
```

修正後:
```rust
//! Individual builtin Box implementations (easy deletion for Plugin migration)
//! ...
//! ConsoleBox: Phase 125 ✅ DELETED - Plugin-only
//! 6. (deleted)
```

### 4. src/boxes/console_box.rs は削除しない

⚠️ **重要**: `src/boxes/console_box.rs` は削除 **しない**！

理由:
- このファイルは **ビルトイン Rust 実装**（src/boxes/ ツリー）
- プラグイン（libnyash_console_plugin.so）が内部で使用している
- VM の直接メソッド呼び出しで使用される（Phase 124 で TypeRegistry ベースの dispatch_by_slot に統合）

```
✅ 保持: src/boxes/console_box.rs（Rust 実装）
❌ 削除: src/box_factory/builtin_impls/console_box.rs（ファクトリー実装）
```

## 実装ステップ

### Step 1: ファイル削除

```bash
# Phase 125: ConsoleBox builtin implementation削除
git rm src/box_factory/builtin_impls/console_box.rs
```

### Step 2: builtin.rs 修正

ファイル: `src/box_factory/builtin.rs`

```diff
match name {
    // Phase 2.1-2.2: DELETE when plugins are confirmed working
    "StringBox" => builtin_impls::string_box::create(args),
    "IntegerBox" => builtin_impls::integer_box::create(args),

    // Phase 2.3: DELETE when BoolBox plugin is created
    "BoolBox" => builtin_impls::bool_box::create(args),

    // Phase 2.4-2.5: DELETE when collection plugins confirmed
    "ArrayBox" => builtin_impls::array_box::create(args),
    "MapBox" => builtin_impls::map_box::create(args),

-   // Phase 2.6: DELETE LAST (critical for logging)
-   "ConsoleBox" => builtin_impls::console_box::create(args),
+   // Phase 125: ✅ DELETED - ConsoleBox is now plugin-only!
+   // See: plugins/nyash-console-plugin for current implementation

    // Phase 15.5: Fallback support (auto/core-ro modes)
    "FileBox" => builtin_impls::file_box::create(args),

    // Phase 113: FileHandleBox Nyash API
    "FileHandleBox" => builtin_impls::filehandle_box::create(args),

    // Special: Keep vs Delete discussion needed
    "NullBox" => builtin_impls::null_box::create(args),

    // Leave other types to other factories (user/plugin)
    _ => Err(RuntimeError::InvalidOperation {
        message: format!("Unknown Box type: {}", name),
    }),
}
```

vec の修正:

```diff
fn box_types(&self) -> Vec<&str> {
    vec![
        // Primitive wrappers
        "StringBox",
        "IntegerBox",
        "BoolBox",
        // Collections/common
        "ArrayBox",
        "MapBox",
-       "ConsoleBox",
+       // ConsoleBox: Phase 125 - Plugin-only (nyash-console-plugin)
        // Fallback support
        "FileBox",
        "FileHandleBox", // Phase 113
        "NullBox",
    ]
}
```

### Step 3: builtin_impls/mod.rs 修正

```diff
//! Individual builtin Box implementations (easy deletion for Plugin migration)
//!
//! Phase 0: ✅ Separated implementations
//! Phase 1-2: 🚧 Deletion roadmap
//!
//! Deletion order:
//! 1. string_box.rs   - Phase 2.1 🔄 Plugin exists, OK to delete
//! 2. integer_box.rs  - Phase 2.2 🔄 Plugin exists, OK to delete
//! 3. bool_box.rs     - Phase 2.3 🔄 Plugin needed first
//! 4. array_box.rs    - Phase 2.4 🔄 Plugin needed first
//! 5. map_box.rs      - Phase 2.5 🔄 Plugin needed first
-//! 6. console_box.rs   - Phase 2.6 🔄 Plugin exists, remove LAST
+//! 6. console_box.rs   - Phase 125 ✅ DELETED! Plugin-only now!
//! 7. file_box.rs     - Phase 15.5-2.7 ⚠️ Fallback support (keep for now)
//! 8. filehandle_box.rs - Phase 113 ✅ Nyash API, keep
//! 9. null_box.rs     - Phase 2.8? ❓ Design decision needed

// Builtin Box factory implementations
pub mod string_box;
pub mod integer_box;
pub mod bool_box;
pub mod array_box;
pub mod map_box;
-pub mod console_box;
+// Phase 125: console_box ✅ DELETED - Plugin-only (nyash-console-plugin)
pub mod file_box;
pub mod filehandle_box;
pub mod null_box;
```

### Step 4: テスト確認

```bash
# コンパイル確認
cargo build --release 2>&1 | grep -E "error"

# ビルトイン ConsoleBox への参照がないことを確認
rg "builtin.*console_box|builtin_impls::console_box" src/ --type rust

# ConsoleBox の参照（プラグインのみ使用）を確認
rg "\"ConsoleBox\"" src/ --type rust | grep -v "comment\|doc\|//\|#"
# → libnyash_console_plugin.so のみが残るはず

# Phase 120 representative tests で plugin ConsoleBox が動作することを確認
NYASH_JOINIR_STRICT=1 ./target/release/nyash apps/tests/esc_dirname_smoke.hako
# → [Console LOG] dir1/dir2 が出力される（プラグイン版 ConsoleBox）
```

### Step 5: フルテスト実行

```bash
# 全テスト実行
cargo test --release 2>&1 | tail -20

# スモークテスト
tools/smokes/v2/run.sh --profile quick
```

## 検証ポイント

### ✅ 検証 1: ビルトイン ConsoleBox が完全に削除されたか

```bash
# builtin_impls::console_box への参照がゼロ
rg "console_box" src/box_factory/ --type rust
# → mod.rs, builtin.rs に残るはずなし（コメント除外）
```

### ✅ 検証 2: プラグイン ConsoleBox が動作するか

```bash
# Phase 120 representative test で ConsoleBox が使用される
NYASH_JOINIR_STRICT=1 ./target/release/nyash apps/tests/esc_dirname_smoke.hako
# 出力: [Console LOG] dir1/dir2
```

### ✅ 検証 3: ビルドエラーがないか

```bash
cargo build --release 2>&1 | grep error
# → ゼロ件（エラーなし）
```

### ✅ 検証 4: テストが失敗していないか

```bash
cargo test --release builtin 2>&1 | grep "test_builtin_console_box_creation"
# → "test not found" か "no matching tests"（テスト削除確認）
```

## 期待される効果

### コード削減
- **ファイル削除**: `console_box.rs` (36行)
- **builtin.rs 削減**: ~10行（case と vec 要素）
- **mod.rs 削減**: ~2行（mod 宣言と コメント更新）
- **テスト削減**: ~10行（test_builtin_console_box_creation）

**合計**: ~58行削減

### アーキテクチャ改善

1. **Plugin-First 原則の実装**: ConsoleBox = プラグインのみ
2. **ビルトイン Factory の簡略化**: 1つの Box 削除
3. **"Everything is Plugin" 完成へ**: Phase 125 で ConsoleBox が最後のマイルストーン
4. **保守性向上**: 単一のプラグイン実装で統一

## 実装上の注意

### 1. ConsoleBox 削除時の段階的調査

削除前に確認:
- nyash.toml で ConsoleBox がプラグイン登録されているか
- `libnyash_console_plugin.so` がビルドされているか
- プラグインが完全に動作するか

```bash
# プラグイン確認
ls plugins/nyash-console-plugin/
ls target/release/libnyash_console_plugin.so

# プラグイン登録確認
rg "ConsoleBox" nyash.toml

# プラグイン動作テスト
echo 'local c = new ConsoleBox(); c.println("Hello")' > test.hako
./target/release/nyash test.hako
```

### 2. ビルトイン ConsoleBox の残存リスク

⚠️ **削除後の確認**:

古いコードが hidden に残っていないか：

```bash
# 削除確認
git log --oneline | head -5  # commit 確認
git status  # 修正ファイル確認
cargo build --release 2>&1 | grep console  # コンパイル確認
```

### 3. テスト との互換性

```bash
# 削除による test の失敗がないか確認
cargo test --release 2>&1 | grep "FAILED\|test failed"
```

## ロールバック計画

修正後に問題が発生した場合：

```bash
# 修正前のバージョンに戻す
git reset --hard HEAD~1

# または特定ファイルのみ復元
git checkout HEAD~ -- src/box_factory/builtin_impls/console_box.rs
git checkout HEAD~ -- src/box_factory/builtin.rs
git checkout HEAD~ -- src/box_factory/builtin_impls/mod.rs
```

## 所要時間

**3時間程度**

- ファイル削除と修正: 1時間
- テスト・検証: 1時間
- ドキュメント更新: 1時間

## 完了後の次のステップ

Phase 126: ドキュメント統合（2時間）

---

**進捗記録**:
- Phase 122.5: nyash.toml method_id 修正 ✅ 完了
- Phase 123: ConsoleBox WASM/非WASM 統一化 ✅ 完了
- Phase 124: VM Method Dispatch 統一化 ✅ 完了
- Phase 125: 削除：deprecated builtin ConsoleBox ← **現在のフェーズ**
- Phase 126: ドキュメント統合（予定）

## 注記

### なぜ src/boxes/console_box.rs は削除しないのか？

```
src/boxes/console_box.rs     ← これは「実装」（削除しない）
   ↓
libnyash_console_plugin.so   ← これは「プラグイン」（存在・使用）
   ↓
src/box_factory/builtin_impls/console_box.rs ← これは「ファクトリー」（削除 = Phase 125）
```

つまり：
- **src/boxes/**: Rust 実装（VM が内部的に使用）
- **plugins/**: プラグイン（ユーザー向けインターフェース）
- **src/box_factory/builtin_impls/**: ビルトイン factory（今回削除）

Phase 125 では **factory** のみ削除し、Rust 実装は残す。
Status: Historical
