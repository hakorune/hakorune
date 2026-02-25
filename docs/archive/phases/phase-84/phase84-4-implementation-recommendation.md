# Phase 84-4: 実装推奨 — BoxCall 型情報登録による根本解決

## 実装優先順位

### 🎯 Phase 84-4-A: dev フォールバック（推奨: 即実装）

**目的**: 開発環境の即座のアンブロック

**実装時間**: 0.5日

**実装箇所**: `src/mir/builder/lifecycle.rs`

```rust
// lifecycle.rs の infer_type_from_phi() の Case D セクション内

// 既存のコード:
// Case D: GenericTypeResolver も失敗 → if_phi フォールバックが必要
eprintln!("[phase82/phi_fallback] Case D triggered for {}", ret_val);

// ↓ 以下を追加 ↓

// Phase 84-4-A: dev 環境専用フォールバック
if should_enable_dev_fallback() {
    if is_base_definition_with_missing_type(self.current_function(), ret_val) {
        eprintln!(
            "[phase84/dev_fallback] {} is base definition with missing type → Unknown (dev only)",
            ret_val
        );
        return Ok(MirType::Unknown);
    }
}

// 既存の panic 処理
if std::env::var("NYASH_PHI_FALLBACK_DISABLED").is_ok() {
    panic!(...);
}
```

**ヘルパー関数**:
```rust
/// Phase 84-4-A: dev 環境専用フォールバック判定
fn should_enable_dev_fallback() -> bool {
    std::env::var("NYASH_PHI_DEV_FALLBACK").ok().as_deref() == Some("1")
}

/// base 定義（BoxCall/Await/etc）で型が未登録かチェック
fn is_base_definition_with_missing_type(
    func: &MirFunction,
    val: ValueId,
) -> bool {
    // val を定義する命令を探索
    for bb in func.blocks.values() {
        for inst in bb.instructions.iter().chain(bb.terminator.iter()) {
            match inst {
                MirInstruction::BoxCall { dst: Some(d), .. }
                | MirInstruction::Await { dst: d, .. }
                | MirInstruction::PluginInvoke { dst: Some(d), .. }
                | MirInstruction::ExternCall { dst: Some(d), .. }
                    if *d == val =>
                {
                    return true;
                }
                _ => {}
            }
        }
    }
    false
}
```

**使用方法**:
```bash
# dev 環境での作業
NYASH_PHI_DEV_FALLBACK=1 cargo test --release --lib

# production 環境（CI）
# 環境変数なし → 依然として厳格なエラー
cargo test --release --lib
```

**利点**:
- ✅ 開発者の作業を即座にアンブロック
- ✅ production 環境は依然として厳格（CI で検出可能）
- ✅ 警告ログで問題箇所を明示

**欠点**:
- ⚠️ 根本解決ではない（暫定措置）
- ⚠️ dev 環境で型エラーが隠蔽される可能性

---

### 🔥 Phase 84-4-B: BoxCall 型情報登録（推奨: 根本解決）

**目的**: BoxCall 戻り値型の完全追跡

**実装時間**: 1-2日

**実装箇所**: `src/mir/builder/builder_calls.rs`

#### ステップ1: 型情報取得インフラ整備

```rust
// builder_calls.rs に追加

/// BoxCall のメソッド戻り値型を推論（Phase 84-4-B）
fn infer_boxcall_return_type(
    &self,
    box_val: ValueId,
    method: &str,
    _args: &[ValueId],
) -> Option<MirType> {
    // 1. box_val の型を取得
    let box_ty = self.value_types.get(&box_val)?;

    // 2. Box 型名を取得
    let box_name = match box_ty {
        MirType::Box { name } => name,
        _ => return None,
    };

    // 3. ビルトイン Box の型情報（ハードコード）
    match (box_name.as_str(), method) {
        // StringBox
        ("StringBox", "upper") => Some(MirType::Box {
            name: "StringBox".to_string(),
        }),
        ("StringBox", "lower") => Some(MirType::Box {
            name: "StringBox".to_string(),
        }),
        ("StringBox", "length") => Some(MirType::Box {
            name: "IntegerBox".to_string(),
        }),

        // IntegerBox
        ("IntegerBox", "abs") => Some(MirType::Box {
            name: "IntegerBox".to_string(),
        }),

        // BoolBox
        ("BoolBox", "not") => Some(MirType::Box {
            name: "BoolBox".to_string(),
        }),

        // ArrayBox
        ("ArrayBox", "length") => Some(MirType::Box {
            name: "IntegerBox".to_string(),
        }),
        ("ArrayBox", "get") => Some(MirType::Unknown), // 要素型は実行時決定

        // Result-like Box (QMark 用)
        (_, "isOk") => Some(MirType::Box {
            name: "BoolBox".to_string(),
        }),
        (_, "getValue") => Some(MirType::Unknown), // Result<T> の T

        // 未知のメソッド
        _ => {
            if std::env::var("NYASH_BOXCALL_TYPE_DEBUG").is_ok() {
                eprintln!(
                    "[boxcall_type] unknown method {}.{} → Unknown",
                    box_name, method
                );
            }
            Some(MirType::Unknown)
        }
    }
}
```

#### ステップ2: emit_box_call() への統合

```rust
// builder_calls.rs の emit_box_call() を修正

pub fn emit_box_call(
    &mut self,
    box_val: ValueId,
    method: &str,
    args: Vec<ValueId>,
) -> Result<ValueId, String> {
    let dst = self.next_value_id();

    // 既存の BoxCall 命令生成
    self.emit_instruction(MirInstruction::BoxCall {
        dst: Some(dst),
        box_val,
        method: method.to_string(),
        args: args.clone(),
        method_id: None,
        effects: EffectMask::UNKNOWN,
    })?;

    // **Phase 84-4-B 新機能**: 戻り値型を推論して登録
    if let Some(ret_ty) = self.infer_boxcall_return_type(box_val, method, &args) {
        self.value_types.insert(dst, ret_ty);

        if std::env::var("NYASH_BOXCALL_TYPE_TRACE").is_ok() {
            eprintln!(
                "[boxcall_type] registered {} = BoxCall({}, {}) → {:?}",
                dst, box_val, method, ret_ty
            );
        }
    }

    Ok(dst)
}
```

#### ステップ3: テスト実行

```bash
# 型推論トレース有効化
NYASH_BOXCALL_TYPE_TRACE=1 cargo test --release --lib mir_lowering_of_qmark_propagate

# 期待される出力:
# [boxcall_type] registered %3 = BoxCall(%1, isOk) → Box(BoolBox)
# [boxcall_type] registered %7 = BoxCall(%1, getValue) → Unknown

# Case D チェック
NYASH_PHI_FALLBACK_DISABLED=1 cargo test --release --lib 2>&1 | grep "Case D" | wc -l
# 期待: 1（await のみ残存）
```

**利点**:
- ✅ 3件（GroupB 2件 + GroupD 1件）を根本解決
- ✅ production 環境でも安全
- ✅ 型情報の追跡可能性向上

**将来の拡張**:
- Phase 26-A で slot_registry から動的に型情報取得
- ユーザー定義 Box のメソッド戻り値型も追跡可能

---

### ⚡ Phase 84-4-C: Await 型情報特殊処理（推奨: 暫定対応）

**目的**: Await 戻り値型の暫定登録

**実装時間**: 0.5日

**実装箇所**: `src/mir/builder/stmts.rs`

```rust
// stmts.rs の build_await_expression() を修正

pub(super) fn build_await_expression(
    &mut self,
    expression: ASTNode,
) -> Result<ValueId, String> {
    let future_value = self.build_expression(expression)?;
    self.emit_instruction(MirInstruction::Safepoint)?;

    let result_id = self.next_value_id();

    // **Phase 84-4-C 新機能**: Future の型から戻り値型を推論
    if let Some(future_ty) = self.value_types.get(&future_value) {
        match future_ty {
            MirType::Box { name } if name.contains("Future") => {
                // Future<T> の T を抽出（Phase 67+ で完全実装予定）
                // 現時点では Unknown として登録
                self.value_types.insert(result_id, MirType::Unknown);

                if std::env::var("NYASH_AWAIT_TYPE_TRACE").is_ok() {
                    eprintln!(
                        "[await_type] registered {} = Await({}) → Unknown (temp)",
                        result_id, future_value
                    );
                }
            }
            _ => {
                // Future 型でない場合も Unknown で登録（エラー防止）
                self.value_types.insert(result_id, MirType::Unknown);
            }
        }
    } else {
        // future_value の型が不明でも Unknown で登録
        self.value_types.insert(result_id, MirType::Unknown);
    }

    self.emit_instruction(MirInstruction::Await {
        dst: result_id,
        future: future_value,
    })?;
    self.emit_instruction(MirInstruction::Safepoint)?;
    Ok(result_id)
}
```

**テスト実行**:
```bash
# 型推論トレース有効化
NYASH_AWAIT_TYPE_TRACE=1 cargo test --release --lib test_lowering_await_expression

# Case D チェック
NYASH_PHI_FALLBACK_DISABLED=1 cargo test --release --lib 2>&1 | grep "Case D" | wc -l
# 期待: 0（全件解決）
```

**利点**:
- ✅ GroupC（1件）を暫定解決
- ✅ Phase 67+ 実装までの橋渡し

**長期対応**:
- Phase 67+ で Future<T> の T 型を正確に抽出
- async/await 型システムの完全実装

---

## 実装順序の推奨

### パターン1: 最速アンブロック（推奨: 即実装）

```
Phase 84-4-A (0.5日)
  ↓
開発作業継続可能
  ↓
Phase 84-4-B (1-2日) + Phase 84-4-C (0.5日)
  ↓
Phase 84-5: if_phi.rs 削除
```

**利点**:
- ✅ 即座に開発環境アンブロック
- ✅ 根本解決と並行作業可能

**欠点**:
- ⚠️ dev 環境で型エラーが一時的に隠蔽

### パターン2: 完璧主義（推奨: 時間に余裕がある場合）

```
Phase 84-4-B (1-2日) + Phase 84-4-C (0.5日)
  ↓
Phase 84-5: if_phi.rs 削除
```

**利点**:
- ✅ dev フォールバック不要
- ✅ 最初から根本解決

**欠点**:
- ⚠️ 実装完了まで開発ブロック（1-2日）

---

## 完了条件と検証方法

### Phase 84-4-A 完了

```bash
# dev 環境での全テスト通過
NYASH_PHI_DEV_FALLBACK=1 NYASH_PHI_FALLBACK_DISABLED=1 cargo test --release --lib 2>&1 | grep "Case D"
# 期待: 出力なし（全件通過）

# production 環境では依然としてエラー
NYASH_PHI_FALLBACK_DISABLED=1 cargo test --release --lib 2>&1 | grep "Case D" | wc -l
# 期待: 4（依然として厳格）
```

### Phase 84-4-B 完了

```bash
# BoxCall 型登録の確認
NYASH_BOXCALL_TYPE_TRACE=1 cargo test --release --lib mir_lowering_of_qmark_propagate 2>&1 | grep "boxcall_type"
# 期待: [boxcall_type] registered ... の出力

# Case D 削減確認
NYASH_PHI_FALLBACK_DISABLED=1 cargo test --release --lib 2>&1 | grep "Case D" | wc -l
# 期待: 1（await のみ残存）
```

### Phase 84-4-C 完了

```bash
# Await 型登録の確認
NYASH_AWAIT_TYPE_TRACE=1 cargo test --release --lib test_lowering_await_expression 2>&1 | grep "await_type"
# 期待: [await_type] registered ... の出力

# Case D 完全解決確認
NYASH_PHI_FALLBACK_DISABLED=1 cargo test --release --lib 2>&1 | grep "Case D" | wc -l
# 期待: 0（全件解決）
```

### Phase 84-5 準備完了

```bash
# if_phi.rs 削除前の最終確認
NYASH_PHI_FALLBACK_DISABLED=1 cargo test --release --lib
# 期待: 全テスト通過（Case D panic なし）

# レガシーフォールバック使用確認
cargo test --release --lib 2>&1 | grep "infer_type_from_phi_fallback"
# 期待: 出力なし（もう使われていない）
```

---

## まとめ

**推奨実装パス**: Phase 84-4-A → Phase 84-4-B → Phase 84-4-C

**総実装時間**: 2-3日

**期待成果**:
- ✅ Case D 4件 → 0件（100%削減）
- ✅ if_phi.rs レガシーフォールバック削除準備完了
- ✅ 型推論システムの完全箱化達成

**次のステップ**: Phase 84-5（if_phi.rs 完全削除）
Status: Historical
