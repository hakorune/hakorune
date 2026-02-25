# Phase 84-3: 残り 4件 Case D の完全調査

## 概要

Phase 84-3 で PhiTypeResolver を実装した結果、Case D は **9件 → 4件に削減**されました（**56%削減達成！**）。

本ドキュメントは残り 4件の詳細分析と、Phase 84-4 での完全削除ロードマップを提示します。

## 削減実績サマリー

| Phase | 実装内容 | Case D 件数 | 削減率 |
|-------|---------|------------|--------|
| Phase 84-1 (Initial) | フォールバック検出実装 | 12件 | - |
| Phase 84-2 | CopyTypePropagator 実装 | 9件 | 25% |
| **Phase 84-3** | **PhiTypeResolver 実装** | **4件** | **56%** |

## 残り 4件の一覧

| # | テスト名 | ValueId | 変更 |
|---|---------|---------|------|
| 1 | `test_lowering_await_expression` | ValueId(2) | 継続 |
| 2 | `mir_lowering_of_qmark_propagate` | ValueId(7) | **新規** |
| 3 | `mir_stage1_cli_emit_program_min_compiles_and_verifies` | ValueId(7) | 継続 |
| 4 | `mir_stage1_cli_emit_program_min_exec_hits_type_error` | ValueId(7) | 継続 |

### Phase 84-3 で解決された 5件（GroupA ループ系）

PhiTypeResolver により以下が解決されました:

- ✅ `loop_with_continue_and_break_edge_copy_merge` - ValueId(56)
- ✅ `nested_loop_with_multi_continue_break_edge_copy_merge` - ValueId(135)
- ✅ `loop_inner_if_multilevel_edge_copy` - ValueId(74)
- ✅ `loop_break_and_early_return_edge_copy` - ValueId(40)
- ✅ `vm_exec_break_inside_if` - ValueId(27)

**削減原因**: Copy → PHI → Copy チェーンを DFS で遡り、base 型定義を発見する能力

## 残り 4件の詳細分析

### 1. test_lowering_await_expression (GroupC: await 特殊構文)

**テストファイル**: `src/mir/mod.rs:363-384`

**コード**:
```rust
let ast = ASTNode::AwaitExpression {
    expression: Box::new(ASTNode::Literal {
        value: LiteralValue::Integer(1),
        span: crate::ast::Span::unknown(),
    }),
    span: crate::ast::Span::unknown(),
};
```

**Lowering 実装**: `src/mir/builder/stmts.rs:388-401`
```rust
pub(super) fn build_await_expression(
    &mut self,
    expression: ASTNode,
) -> Result<ValueId, String> {
    let future_value = self.build_expression(expression)?;
    self.emit_instruction(MirInstruction::Safepoint)?;
    let result_id = self.next_value_id();
    self.emit_instruction(MirInstruction::Await {
        dst: result_id,
        future: future_value,
    })?;
    self.emit_instruction(MirInstruction::Safepoint)?;
    Ok(result_id)
}
```

**問題の本質**:
- `Await { dst: ValueId(2), future: ValueId(1) }` 命令の戻り値型が未登録
- `ValueId(1)` は `Integer(1)` の型 (IntegerBox) が登録済み
- しかし、`Await` 命令の戻り値型は **Future の解決値の型** であり、
  現在の MIR では型情報が失われている

**なぜ PhiTypeResolver で解決できないか**:
- `ValueId(2)` は `Await` 命令の dst（base 定義）
- value_types に型が登録されていない
- PHI/Copy チェーンではないため、探索しても型が見つからない

**解決策**:
1. **短期**: Await 命令の戻り値を Unknown として許容
2. **中期**: Await 命令 lowering 時に future の型から戻り値型を推論
3. **長期**: Phase 67+ async/await 型システム実装

### 2. mir_lowering_of_qmark_propagate (GroupD: QMark 特殊構文) **新規失敗**

**テストファイル**: `src/tests/mir_qmark_lower.rs:5-33`

**コード**:
```rust
let ast = ASTNode::QMarkPropagate {
    expression: Box::new(ASTNode::New {
        class: "StringBox".to_string(),
        arguments: vec![ASTNode::Literal {
            value: crate::ast::LiteralValue::String("ok".to_string()),
            span: Span::unknown(),
        }],
        type_arguments: vec![],
        span: Span::unknown(),
    }),
    span: Span::unknown(),
};
```

**Lowering 実装**: `src/mir/builder/exprs_qmark.rs:6-42`
```rust
pub(super) fn build_qmark_propagate_expression(
    &mut self,
    expression: ASTNode,
) -> Result<ValueId, String> {
    let res_val = self.build_expression_impl(expression)?;
    let res_local = self.local_ssa_ensure(res_val, 0);
    let ok_id = self.next_value_id();
    self.emit_instruction(super::MirInstruction::BoxCall {
        dst: Some(ok_id),
        box_val: res_local,
        method: "isOk".to_string(),
        args: vec![],
        method_id: None,
        effects: super::EffectMask::PURE,
    })?;
    let then_block = self.block_gen.next();
    let else_block = self.block_gen.next();
    let ok_local = self.local_ssa_ensure(ok_id, 4);
    crate::mir::builder::emission::branch::emit_conditional(
        self, ok_local, then_block, else_block,
    )?;
    self.start_new_block(then_block)?;
    self.emit_instruction(super::MirInstruction::Return {
        value: Some(res_local),
    })?;
    self.start_new_block(else_block)?;
    let val_id = self.next_value_id();
    self.emit_instruction(super::MirInstruction::BoxCall {
        dst: Some(val_id),
        box_val: res_local,
        method: "getValue".to_string(),
        args: vec![],
        method_id: None,
        effects: super::EffectMask::PURE,
    })?;
    Ok(val_id)
}
```

**制御フロー構造**:
```
Block1:
  %res = new StringBox("ok")
  %ok = BoxCall(%res, "isOk")
  br %ok, then_block, else_block

Block2 (then_block):
  ret %res

Block3 (else_block):
  %val = BoxCall(%res, "getValue")  ← ValueId(7) の型が未登録
  // 暗黙の return %val
```

**問題の本質**:
- `BoxCall { dst: ValueId(7), method: "getValue" }` の戻り値型が未登録
- `getValue()` の戻り値型は **Result<T> の T 型** だが、型情報が失われている
- main 関数の return 型を推論する際に ValueId(7) の型が必要

**なぜ PhiTypeResolver で解決できないか**:
- `ValueId(7)` は `BoxCall` 命令の dst（base 定義）
- value_types に型が登録されていない
- PHI/Copy チェーンではないため、探索しても型が見つからない

**Phase 84-2 で失敗していなかった理由**:
- 以前の調査文書には記載なし → **PhiTypeResolver 実装の副作用で新たに顕在化**
- 可能性1: 以前は別の型推論経路で偶然解決していた
- 可能性2: テスト自体が無効化されていた（要調査）

**解決策**:
1. **短期**: BoxCall の戻り値型を Unknown として許容（dev 環境のみ）
2. **中期**: BoxCall 命令 lowering 時にメソッド型情報から戻り値型を登録
3. **長期**: Phase 26-A ValueKind 型安全化で BoxCall 戻り値型を完全追跡

### 3-4. mir_stage1_cli_emit_program_min_* (GroupB: Stage1Cli 複雑型推論)

**テストファイル**: `src/tests/mir_stage1_cli_emit_program_min.rs:71-138`

**コード**: 138行の複雑な static box 定義
```hako
static box Stage1Cli {
  emit_program_json(source) {
    if source == null || source == "" { return null }
    return "{prog:" + source + "}"
  }

  stage1_main(args) {
    local src = env.get("STAGE1_SOURCE")
    if src == null || src == "" { return 96 }
    local prog = me.emit_program_json(src)
    if prog == null { return 96 }
    print(prog)
    return 0
  }
}

static box Main {
  main(args) {
    env.set("STAGE1_SOURCE", "apps/tests/stage1_using_minimal.hako")
    return Stage1Cli.stage1_main(args) // ← ValueId(7) の型
  }
}
```

**問題の本質**:
- 多段メソッド呼び出し: `Main.main` → `Stage1Cli.stage1_main` → `emit_program_json`
- 複数の return 経路: null / 96 / 0
- PHI が複数の経路から合流するが、**BoxCall の戻り値型が未登録**

**なぜ PhiTypeResolver で解決できないか**:
- `ValueId(7)` は `BoxCall { method: "stage1_main" }` の dst（base 定義）
- value_types に型が登録されていない
- PHI/Copy チェーンではないため、探索しても型が見つからない

**デバッグ情報**:
```
[DEBUG/build_block] Completed, returning value ValueId(14)
[DEBUG/build_block] Completed, returning value ValueId(83)
[DEBUG/build_block] Completed, returning value ValueId(95)
[DEBUG/build_block] Completed, returning value ValueId(47)
[DEBUG/build_block] Completed, returning value ValueId(63)
[DEBUG/build_block] Completed, returning value ValueId(7)
```

多数の ValueId が生成されており、PHI 合流が複雑であることを示唆。

**解決策**:
1. **短期**: BoxCall の戻り値型を Unknown として許容（dev 環境のみ）
2. **中期**: BoxCall 命令 lowering 時にメソッド型情報から戻り値型を登録
3. **長期**: Phase 26-A ValueKind 型安全化で BoxCall 戻り値型を完全追跡

## パターン分類の再整理

Phase 84-3 の結果を踏まえ、パターン分類を更新:

### GroupA: Loop 制御フロー PHI（5件 → 0件） ✅ 完全解決

**PhiTypeResolver で解決**: Copy → PHI → Copy チェーンを DFS で遡る能力

### GroupB: Stage1Cli 複雑型推論（2件 → 2件） ⚠️ 継続

**根本原因**: BoxCall 戻り値型の未登録（base 定義の型情報欠落）

### GroupC: await 特殊構文（1件 → 1件） ⚠️ 継続

**根本原因**: Await 戻り値型の未登録（base 定義の型情報欠落）

### GroupD: QMark 特殊構文（0件 → 1件） ⚠️ 新規出現

**根本原因**: BoxCall 戻り値型の未登録（getValue メソッドの型情報欠落）

## Phase 84-4 で必要な機能の推奨

### 推奨1: BoxCall 戻り値型の実行時登録（優先度: 最高）

**対象**: GroupB（2件）、GroupD（1件）

**問題の統一化**:
- 全て「BoxCall の dst が value_types に未登録」という同一問題
- PhiTypeResolver では base 定義の型を発見できない

**解決策（2段階）**:

#### Phase 84-4-A: 暫定フォールバック（dev 環境専用）

**実装箇所**: `src/mir/builder/lifecycle.rs`

```rust
// lifecycle.rs の infer_type_from_phi() 内
if should_enable_dev_fallback() {
    // dev 環境専用: BoxCall/Await の戻り値型を Unknown として許容
    if is_boxcall_or_await_result(function, ret_val) {
        eprintln!(
            "[phase84/dev_fallback] BoxCall/Await result {} → Unknown (dev only)",
            ret_val
        );
        return Ok(MirType::Unknown);
    }
}
```

**環境変数制御**:
```rust
fn should_enable_dev_fallback() -> bool {
    std::env::var("NYASH_PHI_DEV_FALLBACK").ok().as_deref() == Some("1")
}
```

**期待効果**:
- 3件 → 1件（await のみ残存）
- dev 環境でのビルド通過
- production 環境では依然として厳格なエラー

#### Phase 84-4-B: BoxCall 型情報の実行時登録（根本解決）

**実装箇所**: `src/mir/builder/builder_calls.rs`

```rust
// emit_box_call() 内に追加
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
        args,
        method_id: None,
        effects: EffectMask::UNKNOWN,
    })?;

    // **新機能**: メソッド型情報から戻り値型を推論して登録
    if let Some(method_ty) = self.infer_boxcall_return_type(box_val, method, &args) {
        self.value_types.insert(dst, method_ty);
    }

    Ok(dst)
}

fn infer_boxcall_return_type(
    &self,
    box_val: ValueId,
    method: &str,
    args: &[ValueId],
) -> Option<MirType> {
    // 1. box_val の型を取得
    let box_ty = self.value_types.get(&box_val)?;

    // 2. method の型情報を slot_registry から取得
    if let Some(slot_id) = self.current_slot_registry
        .as_ref()
        .and_then(|reg| reg.resolve_method(box_ty, method))
    {
        // 3. slot_id からメソッドシグネチャを取得
        // （Phase 26-A で実装予定）
        return Some(MirType::Unknown); // 暫定
    }

    None
}
```

**期待効果**:
- 3件 → 1件（await のみ残存）
- production 環境でも安全に動作
- 型情報の追跡可能性向上

### 推奨2: Await 型情報の特殊処理（優先度: 中）

**対象**: GroupC（1件）

**短期対応**: Phase 84-4-A の dev フォールバックで対応（Unknown として許容）

**長期対応**: Phase 67+ async/await 型システム実装
```rust
// build_await_expression() 内に追加
pub(super) fn build_await_expression(
    &mut self,
    expression: ASTNode,
) -> Result<ValueId, String> {
    let future_value = self.build_expression(expression)?;
    self.emit_instruction(MirInstruction::Safepoint)?;

    let result_id = self.next_value_id();

    // **新機能**: Future の型から戻り値型を推論
    if let Some(future_ty) = self.value_types.get(&future_value) {
        if let MirType::Box { name } = future_ty {
            if name.contains("Future") {
                // Future<T> の T を抽出（Phase 67+ で実装）
                let resolved_ty = extract_future_inner_type(name);
                self.value_types.insert(result_id, resolved_ty);
            }
        }
    }

    self.emit_instruction(MirInstruction::Await {
        dst: result_id,
        future: future_value,
    })?;
    self.emit_instruction(MirInstruction::Safepoint)?;
    Ok(result_id)
}
```

## Phase 84-4 実装ロードマップ

### Phase 84-4-A: dev フォールバック実装（0.5日）

**目標**: 開発環境でのビルド通過

**ステップ**:
1. `lifecycle.rs` に `is_boxcall_or_await_result()` 実装
2. `should_enable_dev_fallback()` 環境変数チェック実装
3. dev フォールバック警告ログ追加
4. テスト実行: `NYASH_PHI_DEV_FALLBACK=1` で 4件 → 0件 確認

**成果**:
- dev 環境での即座のアンブロック
- production 環境は依然として厳格

### Phase 84-4-B: BoxCall 型情報登録（1-2日）

**目標**: BoxCall 戻り値型の根本解決

**ステップ**:
1. `builder_calls.rs` に `infer_boxcall_return_type()` 実装
2. `emit_box_call()` 内で戻り値型を value_types に登録
3. slot_registry とのインテグレーション
4. テスト実行: 3件 → 1件（await のみ残存）確認

**成果**:
- GroupB（2件）完全解決
- GroupD（1件）完全解決
- production 環境でも安全

### Phase 84-4-C: Await 型情報特殊処理（0.5日）

**目標**: Await 戻り値型の暫定対応

**ステップ**:
1. `build_await_expression()` に Future 型チェック追加
2. Unknown 型での暫定登録
3. テスト実行: 1件 → 0件 確認

**成果**:
- GroupC（1件）暫定解決
- Phase 67+ 実装までの橋渡し

## 完了条件

```bash
# Phase 84-4-A 完了
NYASH_PHI_DEV_FALLBACK=1 NYASH_PHI_FALLBACK_DISABLED=1 cargo test --release --lib 2>&1 | grep "Case D" | wc -l
# 期待: 0（dev フォールバックで全件通過）

# Phase 84-4-B 完了
NYASH_PHI_FALLBACK_DISABLED=1 cargo test --release --lib 2>&1 | grep "Case D" | wc -l
# 期待: 1（await のみ残存）

# Phase 84-4-C 完了
NYASH_PHI_FALLBACK_DISABLED=1 cargo test --release --lib 2>&1 | grep "Case D" | wc -l
# 期待: 0（全件解決）
```

## if_phi.rs レガシー削除ロードマップ

### Phase 84-5: レガシーフォールバック完全削除（1日）

**前提条件**: Phase 84-4-C 完了（Case D = 0件）

**ステップ**:
1. `src/mir/join_ir/lowering/if_phi.rs` 完全削除
2. `GenericTypeResolver` の if_phi 呼び出し削除
3. `lifecycle.rs` の Case D 処理を全て削除
4. 全テスト実行: Case D panic が 0件であることを確認
5. ドキュメント更新: Phase 82-84 完了宣言

**期待成果**:
- if_phi.rs（約 300行）完全削除
- 型推論システムの完全箱化達成
- レガシーフォールバック根絶

### 削除による技術的利点

1. **箱理論の完全実現**:
   - PhiTypeResolver: PHI + Copy グラフ専用箱
   - CopyTypePropagator: Copy 型伝播専用箱
   - GenericTypeResolver: 統合調整箱
   - if_phi.rs: 削除（レガシー汚染源の根絶）

2. **保守性向上**:
   - 型推論ロジックが 3箱に明確分離
   - 各箱の責務が単一明確
   - 新規型推論パターン追加が容易

3. **パフォーマンス改善**:
   - if_phi.rs の非効率な全探索削除
   - PhiTypeResolver の DFS による効率的探索
   - value_types キャッシュの最適化

## まとめ

**Phase 84-3 の成果**:
- PhiTypeResolver 実装により 9件 → 4件（56%削減）
- GroupA（Loop 制御フロー）5件を完全解決

**残り 4件の本質**:
- 全て「base 定義（BoxCall/Await）の型情報欠落」という同一問題
- PhiTypeResolver では解決不可能（設計上正しい制約）

**Phase 84-4 の戦略**:
1. **Phase 84-4-A**: dev フォールバック実装（0.5日）
2. **Phase 84-4-B**: BoxCall 型情報登録（1-2日）
3. **Phase 84-4-C**: Await 型情報特殊処理（0.5日）

**Phase 84-5 の目標**:
- if_phi.rs レガシーフォールバック完全削除
- 型推論システムの完全箱化達成
- Phase 82-84 完全達成宣言

**総削減見込み**:
- 12件（初期）→ 0件（Phase 84-5 完了時）
- **100%削減達成！**
Status: Historical
