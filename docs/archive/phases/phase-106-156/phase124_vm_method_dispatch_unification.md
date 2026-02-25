# Phase 124: VM Method Dispatch 統一化

⚠️ **Note**: このドキュメントは Phase 124 の実装記録です。
           統合的なガイドは [ConsoleBox 完全ガイド](consolebox_complete_guide.md) をご参照ください。

## 目的

`src/backend/mir_interpreter/handlers/calls/method.rs` の `execute_method_call()` を、現在の手動 match ベースの分岐処理から **TypeRegistry ベースの統一的なディスパッチ** に移行する。

これにより、ビルトイン型（String, Integer, Array）とプラグイン Box のメソッド解決を統一化し、コードの保守性と拡張性を向上させる。

## 現在の問題

### パターン 1: 型ごとの match 分岐（lines 155-216）

```rust
fn execute_method_call(
    &mut self,
    receiver: &VMValue,
    method: &str,
    args: &[ValueId],
) -> Result<VMValue, VMError> {
    match receiver {
        VMValue::String(s) => match method {
            "length" => Ok(VMValue::Integer(s.len() as i64)),
            "concat" => { /* 実装 */ }
            "replace" => { /* 実装 */ }
            // ... 複数のメソッド
        },
        VMValue::BoxRef(box_ref) => {
            // ConsoleBox特別処理
            if box_ref.type_name() == "ConsoleBox" { ... }
            // StringBox特別処理
            if box_ref.type_name() == "StringBox" { ... }
            // プラグイン処理
            if let Some(p) = box_ref.as_any().downcast_ref::<PluginBoxV2>() { ... }
        },
        _ => Err(...)
    }
}
```

### 問題点

1. **統一性の欠如**: String のメソッドと BoxRef のメソッドで異なる処理パス
2. **スケーラビリティの悪さ**: 新しい型を追加するたびに新しい match arm が必要
3. **二重実装の危険性**: StringBox と String の両方にメソッド実装がある可能性
4. **保守性**: TypeRegistry と VM 実装が乖離

### パターン 2: Box 型の重複処理（lines 218-340）

```rust
// ConsoleBox の特別処理（lines 220-265）
if box_ref.type_name() == "ConsoleBox" {
    if let Some(console) = box_ref.as_any().downcast_ref::<ConsoleBox>() {
        match method {
            "log" | "println" => { /* Rust 実装 */ }
            // ...
        }
    }
}

// StringBox の特別処理（lines 267-317）
if box_ref.type_name() == "StringBox" {
    let s_box = box_ref.to_string_box();
    let s = s_box.value;
    match method {
        "lastIndexOf" => { /* 実装 */ }
        // ...
    }
}

// プラグイン Box の処理（lines 318-340）
if let Some(p) = box_ref.as_any().downcast_ref::<PluginBoxV2>() {
    let host = get_global_plugin_host();
    match host.invoke_instance_method(...) {
        Ok(Some(ret)) => Ok(VMValue::from_nyash_box(ret)),
        // ...
    }
}
```

**問題**: ビルトイン Box（ConsoleBox, StringBox, ArrayBox）とプラグイン Box で異なる処理パス

## 設計: TypeRegistry ベースの統一ディスパッチ

### 目標アーキテクチャ

```
┌─────────────────────────────────────────────────────┐
│ execute_method_call(receiver, method_name, args)    │
│                                                      │
│  1. receiver から type_name を取得                   │
│  2. TypeRegistry で type_id を検索                   │
│  3. MethodEntry から slot を取得                     │
│  4. 統一ディスパッチ (dispatch_by_slot)              │
│     └─> 引数パッキング & invoke                     │
│     └─> 結果のアンパック                            │
└─────────────────────────────────────────────────────┘
```

### 実装ステップ

#### Step 1: TypeRegistry へのビルトイン型登録

**ファイル**: `src/runtime/type_registry.rs`

現在、TypeRegistry には Box 型のみ登録（type_id 1-7）。String, Integer などのビルトイン型は登録されていない。

**修正内容**：

```rust
// type_registry.rs の定義部で、ビルトイン型用の定義を追加

// ビルトイン型の type_id を確認
pub const BUILTIN_STRING_TYPE_ID: u32 = 100;  // 任意の ID
pub const BUILTIN_INTEGER_TYPE_ID: u32 = 101;
pub const BUILTIN_ARRAY_TYPE_ID: u32 = 102;

// 初期化時に TypeRegistry へ登録
pub fn register_builtin_types() {
    // String メソッド
    let string_methods = vec![
        ("length", 0, 1),
        ("concat", 1, 2),
        ("replace", 2, 3),
        ("indexOf", 3, 4),
        ("lastIndexOf", 4, 5),
        ("substring", 5, 6),
    ];
    // ... 登録処理

    // Integer メソッド
    let integer_methods = vec![
        ("to_string", 10, 11),
        // ... 他のメソッド
    ];
    // ... 登録処理

    // Array メソッド
    let array_methods = vec![
        ("birth", 30, 31),
        ("push", 31, 32),
        ("length", 32, 33),
        ("get", 33, 34),
        ("set", 34, 35),
    ];
    // ... 登録処理
}
```

#### Step 2: ディスパッチ関数の実装

**ファイル**: `src/backend/mir_interpreter/handlers/calls/method.rs`

```rust
/// TypeRegistry ベースの統一ディスパッチ
fn dispatch_by_slot(
    &mut self,
    receiver: &VMValue,
    type_id: u32,
    slot: u32,
    args: &[ValueId],
) -> Result<VMValue, VMError> {
    // type_id と slot に基づいて実装を選択
    match (type_id, slot) {
        // String メソッド
        (BUILTIN_STRING_TYPE_ID, 1) => {
            // "length" メソッド
            if let VMValue::String(s) = receiver {
                Ok(VMValue::Integer(s.len() as i64))
            } else {
                Err(self.err_invalid("String.length: invalid receiver"))
            }
        }
        (BUILTIN_STRING_TYPE_ID, 2) => {
            // "concat" メソッド
            if let VMValue::String(s) = receiver {
                if let Some(arg_id) = args.get(0) {
                    let arg_val = self.reg_load(*arg_id)?;
                    let new_str = format!("{}{}", s, arg_val.to_string());
                    Ok(VMValue::String(new_str))
                } else {
                    Err(self.err_invalid("String.concat: requires 1 argument"))
                }
            } else {
                Err(self.err_invalid("String.concat: invalid receiver"))
            }
        }
        // ... 他の String メソッド

        // Array メソッド
        (BUILTIN_ARRAY_TYPE_ID, 31) => {
            // "push" メソッド
            // ...
        }

        // プラグイン Box（slot >= 1000）
        (_, slot) if slot >= 1000 => {
            // プラグイン Box のメソッド呼び出し
            // ...
        }

        _ => Err(self.err_method_not_found(&format!("type_id={}", type_id), "unknown"))
    }
}
```

#### Step 3: execute_method_call の簡略化

```rust
fn execute_method_call(
    &mut self,
    receiver: &VMValue,
    method: &str,
    args: &[ValueId],
) -> Result<VMValue, VMError> {
    // 1. receiver から type_name を取得
    let type_name = match receiver {
        VMValue::String(_) => "String",
        VMValue::Integer(_) => "Integer",
        VMValue::Bool(_) => "Bool",
        VMValue::Null => "Null",
        VMValue::Void => "Void",
        VMValue::BoxRef(bx) => bx.type_name(),
        VMValue::Handle(_) => "Handle",
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

## 実装優先順位

### Priority 1: String メソッドの統一化（1時間）

- String.length, concat, replace, indexOf, lastIndexOf, substring
- 現在のコード（lines 156-216）を dispatch_by_slot に移行
- テスト: string_ops_basic.hako が動作確認

### Priority 2: Array メソッドの統一化（1時間）

- Array.birth, push, length, get, set
- 現在のコード（lines 88-121）を dispatch_by_slot に移行
- テスト: test_array_simple.hako が動作確認

### Priority 3: Box 型の統一化（1.5時間）

- ConsoleBox.log, println, warn, error, clear
- StringBox.indexOf, find, is_space, is_alpha
- 現在の特別処理（lines 220-317）を dispatch_by_slot に移行
- テスト: Phase 120 representative tests が動作確認

### Priority 4: コード削減（0.5時間）

- 古い match 文の削除
- 未使用の型チェック関数の削除
- ドキュメント更新

## 実装の注意点

### 1. タイプシステムの一貫性

```rust
// ❌ 問題: String は VMValue::String だが、
//         StringBox は VMValue::BoxRef に包含されている
VMValue::String(s)      // プリミティブ
VMValue::BoxRef(bx)     // String "StringBox" という Box

// ✅ 解決: TypeRegistry で両方を同じ type_id で管理
// String: type_id=100（プリミティブ用）
// StringBox: type_id=100（同じ、VM上で互換性）
```

### 2. スロット (slot) の管理

現在、nyash.toml や type_registry.rs で slot が定義されているが、整合性が取れているか確認が必要：

```toml
# nyash.toml の例
[libraries."libnyash_console_plugin.so".ConsoleBox.methods]
log = { method_id = 1 }
println = { method_id = 1 }
warn = { method_id = 401 }  # ← slot 401？
```

**修正が必要な場合**:
- TypeRegistry での slot 定義と nyash.toml を統一
- ドキュメント化（slot 予約表）

### 3. フォールバックの廃止

現在、古いコードには環境変数ガード（`NYASH_VM_RECV_ARG_FALLBACK` など）がある。Phase 124 では削除可能：

```rust
// ❌ 削除対象
let tolerate = std::env::var("NYASH_VM_RECV_ARG_FALLBACK")
    .ok()
    .as_deref()
    == Some("1");
if tolerate { /* フォールバック */ }

// ✅ Phase 124 後: TypeRegistry ベースなので不要
```

## テスト計画

### ユニットテスト

新しい slot ベースのディスパッチを確認：

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_dispatch_by_slot_string_length() {
        // slot = String.length の slot
        let receiver = VMValue::String("hello".to_string());
        let slot = 1;  // type_id=100 での length の slot
        // ... assert
    }
}
```

### 統合テスト

既存のテストすべてが動作確認：

```bash
# 基本テスト
cargo test --release

# Phase 120 representative tests
NYASH_JOINIR_STRICT=1 ./target/release/nyash apps/tests/esc_dirname_smoke.hako

# スモークテスト
tools/smokes/v2/run.sh --profile quick
```

## 期待される効果

### コード削減
- **削減行数**: ~100行（execute_method_call の簡略化）
- **削除対象**: 古い match 文、環境変数ガード

### 保守性向上
- メソッド解決が一元化（TypeRegistry）
- ビルトイン型とプラグイン Box の統一処理
- スロット番号で高速ディスパッチ可能

### 拡張性向上
- 新しい Box 型の追加が容易（TypeRegistry 登録のみ）
- ユーザー定義 Box への拡張が自然

### 性能向上
- slot による直接ディスパッチ（型チェック削減）
- match の arm 数削減

## 実装上の危険

### 1. Type ID の衝突

ビルトイン型（String/Integer など）と Box 型の type_id が衝突しないよう注意：

```rust
// Box 型: type_id = 1-10（既存）
pub const CONSOLE_BOX_TYPE_ID: u32 = 7;

// ビルトイン型: type_id = 100-199（新規）
pub const BUILTIN_STRING_TYPE_ID: u32 = 100;
pub const BUILTIN_INTEGER_TYPE_ID: u32 = 101;
```

### 2. 既存コードとの互換性

Phase 124 の修正後、古い `execute_method_call` のコードパスが残っていないか確認：

```bash
# 古いパターンの検索
rg "VMValue::String.*match method" src/
rg "type_name.*ConsoleBox" src/
```

### 3. テストカバレッジ

Phase 124 後、全メソッドが dispatch_by_slot を通るようになったため、テストも統一的に実行されることを確認。

## ロールバック計画

修正後に問題が発生した場合：

```bash
# 修正前のバージョンに戻す
git checkout HEAD~ -- src/backend/mir_interpreter/handlers/calls/method.rs

# または
git show <commit-hash>:src/... > src/...
```

## 所要時間

**4時間程度**

- Step 1-2 (TypeRegistry 登録 + ディスパッチ関数): 1.5時間
- Step 3 (execute_method_call 簡略化): 1.5時間
- Step 4 (テスト + 削減): 1時間

## 完了後の次のステップ

Phase 125: 削除：deprecated builtin ConsoleBox（3時間）

---

**進捗記録**:
- Phase 122.5: nyash.toml method_id 修正 ✅ 完了
- Phase 123: ConsoleBox WASM/非WASM 統一化 ✅ 完了
- Phase 124: VM Method Dispatch 統一化 ← **現在のフェーズ**
- Phase 125: 削除：deprecated builtin ConsoleBox（予定）
- Phase 126: ドキュメント統合（予定）
Status: Historical
