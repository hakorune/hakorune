# Phase 84-2: Case D 残り 9件の詳細調査

## 概要

Phase 84-2 で CopyTypePropagator を実装した結果、Case D は 12件 → 9件に削減されました。
本ドキュメントは残り 9件の詳細分析結果をまとめます。

## 削減された 3件（Phase 84-2 で解決）

CopyTypePropagator により以下のパターンが解決されました:
- **Copy チェーン伝播**: `r1 = Copy r0` → `r2 = PHI [r1, r3]` で r0 の型が r2 に伝播
- **多段 Copy 追跡**: Copy 命令を遡って元の ValueId の型を発見

## 残り 9件の一覧

| # | テスト名 | ValueId | パターン分類 |
|---|---------|---------|------------|
| 1 | `test_lowering_await_expression` | ValueId(2) | GroupC: await 特殊パターン |
| 2 | `loop_with_continue_and_break_edge_copy_merge` | ValueId(56) | GroupA: Loop + continue/break PHI |
| 3 | `nested_loop_with_multi_continue_break_edge_copy_merge` | ValueId(135) | GroupA: Nested loop 複雑 PHI |
| 4 | `loop_inner_if_multilevel_edge_copy` | ValueId(74) | GroupA: Loop + 多段 if |
| 5 | `loop_break_and_early_return_edge_copy` | ValueId(40) | GroupA: Loop + early return |
| 6 | `vm_exec_break_inside_if` | ValueId(27) | GroupA: Loop + if-break |
| 7 | `loop_if_three_level_merge_edge_copy` | ValueId(75) | GroupA: Loop + 3段 if |
| 8 | `mir_stage1_cli_emit_program_min_exec_hits_type_error` | ValueId(7) | GroupB: Stage1Cli 複雑型推論 |
| 9 | `mir_stage1_cli_emit_program_min_compiles_and_verifies` | ValueId(7) | GroupB: Stage1Cli 複雑型推論 |

## パターン分類の詳細

### GroupA: Loop 制御フロー PHI（7件）

**共通特徴**:
- `NYASH_MIR_NO_PHI=1` でテスト（PHI-off モード）
- Loop + continue/break による複雑な制御フロー
- Edge Copy が複数の経路から合流する PHI

**典型的な制御フロー**:
```
loop(condition) {
  if (cond1) { break }    // → after_loop へ edge copy
  if (cond2) { continue } // → loop_header へ edge copy
  normal_path             // → loop_header へ edge copy
}
return merged_value       // ← PHI の型が未解決
```

**問題の本質**:
- PHI の incoming 値が全て Edge Copy の dst ValueId
- Edge Copy の src ValueId は value_types に登録されている
- しかし、GenericTypeResolver は **PHI の incoming 値の型** しか見ない
- **Copy の src を遡る処理が不足**

**なぜ CopyTypePropagator で解決できなかったか**:
```rust
// CopyTypePropagator の現在のロジック
if let Some(ty) = types.get(&src) {
    types.insert(dst, ty.clone()); // ← dst に型を登録
}
```

問題点:
- PHI の incoming 値 (dst) に型を登録
- しかし、GenericTypeResolver::resolve_from_phi() は **既に登録された型** を参照
- **参照のタイミングが遅い**: lifecycle.rs が return 型を要求する時点で、
  CopyTypePropagator はまだ実行されていない

**具体例: loop_with_continue_and_break_edge_copy_merge**

```hako
// 簡略化したコード
i = 0
sum = 0
loop(i < 5) {
  i = i + 1
  if (i == 3) { break }    // → edge copy: sum_final = sum
  if (i % 2 == 0) { continue } // → edge copy: sum_loop = sum
  sum = sum + i
}
return sum // ← ValueId(56) の型が未解決
```

MIR 構造（推測）:
```
Block1 (loop_header):
  %sum_header = PHI [%sum_init, %sum_loop, %sum_updated]

Block2 (break):
  %sum_final = Copy %sum_header  ← value_types に型登録済み
  Jump after_loop

Block3 (continue):
  %sum_loop = Copy %sum_header   ← value_types に型登録済み
  Jump loop_header

Block4 (after_loop):
  %56 = PHI [%sum_final]         ← incoming の型が未登録！
  Return %56
```

**テストファイルの所在**:
- `src/tests/loop_continue_break_no_phi_tests.rs`
- `src/tests/loop_nested_no_phi_tests.rs`
- `src/tests/loop_return_no_phi_tests.rs`
- `src/tests/mir_ctrlflow_break_continue.rs`

### GroupB: Stage1Cli 複雑型推論（2件）

**共通特徴**:
- `mir_stage1_cli_emit_program_min.rs` の 2テスト
- static box + env.get/set による複雑な型フロー
- ValueId(7) が Main.main の戻り値

**コードの特徴**:
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
- PHI が複数の経路から合流

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

**テストファイル**:
- `src/tests/mir_stage1_cli_emit_program_min.rs`

### GroupC: await 特殊パターン（1件）

**テスト**: `test_lowering_await_expression`
**ValueId**: ValueId(2)

**コードの特徴**:
```rust
// src/mir/mod.rs:363
let ast = ASTNode::AwaitExpression {
    expression: Box::new(ASTNode::Literal {
        value: LiteralValue::Integer(1),
        span: crate::ast::Span::unknown(),
    }),
    span: crate::ast::Span::unknown(),
};
```

**問題の本質**:
- await 式の MIR lowering が特殊な制御フローを生成
- Core-13 pure mode では skip される（非同期システム未実装）
- ValueId(2) は await の戻り値

**なぜ特殊か**:
- await は将来的に Safepoint/Checkpoint 命令に変換される予定
- 現在は簡易実装のため、型推論が不完全

**テストファイル**:
- `src/mir/mod.rs:363-384`

## Phase 84-3 で必要な機能の推奨

### 推奨1: Edge Copy 追跡 PHI 型推論（優先度: 高）

**対象**: GroupA（7件）

**アルゴリズム**:
```rust
// GenericTypeResolver に追加
pub fn resolve_from_phi_with_copy_trace(
    function: &MirFunction,
    ret_val: ValueId,
    types: &BTreeMap<ValueId, MirType>,
) -> Option<MirType> {
    // 1. PHI 命令を探索
    for inst in find_phi_instructions(function, ret_val) {
        if let MirInstruction::Phi { inputs, .. } = inst {
            // 2. incoming 値ごとに Copy を遡る
            let mut inferred_types = Vec::new();
            for (_, incoming_val) in inputs {
                // 2-1. incoming_val の型を直接取得
                if let Some(ty) = types.get(incoming_val) {
                    inferred_types.push(ty.clone());
                    continue;
                }

                // 2-2. incoming_val を定義する Copy 命令を探索
                if let Some(src_val) = find_copy_src(function, *incoming_val) {
                    if let Some(ty) = types.get(&src_val) {
                        inferred_types.push(ty.clone());
                        continue;
                    }
                }

                // 2-3. 多段 Copy を再帰的に遡る
                if let Some(ty) = trace_copy_chain(function, *incoming_val, types, 10) {
                    inferred_types.push(ty);
                }
            }

            // 3. 全ての型が一致すれば返す
            if let Some(first) = inferred_types.first() {
                if inferred_types.iter().all(|t| t == first) {
                    return Some(first.clone());
                }
            }
        }
    }
    None
}

fn trace_copy_chain(
    function: &MirFunction,
    start: ValueId,
    types: &BTreeMap<ValueId, MirType>,
    max_depth: usize,
) -> Option<MirType> {
    let mut current = start;
    for _ in 0..max_depth {
        if let Some(ty) = types.get(&current) {
            return Some(ty.clone());
        }
        if let Some(src) = find_copy_src(function, current) {
            current = src;
        } else {
            break;
        }
    }
    None
}
```

**実装箇所**:
- `src/mir/join_ir/lowering/generic_type_resolver.rs`

**期待効果**:
- GroupA の 7件を一気に解決
- Loop + continue/break パターンの完全対応

### 推奨2: 多段 PHI 型推論（優先度: 中）

**対象**: GroupB（2件）

**問題**:
```
Block1:
  %a = PHI [const_96, const_0]

Block2:
  %b = PHI [%a, const_0]

Block3:
  %7 = PHI [%b]  ← %b の型が未解決
```

**アルゴリズム**:
```rust
pub fn resolve_from_phi_recursive(
    function: &MirFunction,
    ret_val: ValueId,
    types: &BTreeMap<ValueId, MirType>,
    visited: &mut HashSet<ValueId>,
) -> Option<MirType> {
    if visited.contains(&ret_val) {
        return None; // 循環検出
    }
    visited.insert(ret_val);

    // 1. 直接型推論を試みる
    if let Some(ty) = resolve_from_phi(function, ret_val, types) {
        return Some(ty);
    }

    // 2. PHI の incoming 値を再帰的に解決
    for inst in find_phi_instructions(function, ret_val) {
        if let MirInstruction::Phi { inputs, .. } = inst {
            let mut inferred_types = Vec::new();
            for (_, incoming_val) in inputs {
                // 再帰的に型を解決
                if let Some(ty) = resolve_from_phi_recursive(
                    function, *incoming_val, types, visited
                ) {
                    inferred_types.push(ty);
                }
            }

            if let Some(first) = inferred_types.first() {
                if inferred_types.iter().all(|t| t == first) {
                    return Some(first.clone());
                }
            }
        }
    }
    None
}
```

**実装箇所**:
- `src/mir/join_ir/lowering/generic_type_resolver.rs`

**期待効果**:
- GroupB の 2件を解決
- 多段メソッド呼び出しの型推論強化

### 推奨3: await 型推論特殊処理（優先度: 低）

**対象**: GroupC（1件）

**短期対応**:
```rust
// lifecycle.rs に特殊ケース追加
if function_name == "main" && is_await_expression {
    // await の戻り値型は Unknown で許容
    return MirType::Unknown;
}
```

**長期対応**:
- Phase 67+ で async/await システム完全実装
- Safepoint/Checkpoint 命令の型推論統合

**実装箇所**:
- `src/mir/builder/lifecycle.rs`

**期待効果**:
- GroupC の 1件を解決（暫定）

## 推奨実装順序

### Phase 84-3: Edge Copy 追跡 PHI 型推論（1-2日）

**目標**: GroupA の 7件を解決

**ステップ**:
1. `GenericTypeResolver::resolve_from_phi_with_copy_trace()` 実装
2. `trace_copy_chain()` ヘルパー関数実装
3. `find_copy_src()` ヘルパー関数実装
4. lifecycle.rs から新関数を呼び出す
5. テスト実行: 7件 → 0件 を確認

### Phase 84-4: 多段 PHI 型推論（1-2日）

**目標**: GroupB の 2件を解決

**ステップ**:
1. `GenericTypeResolver::resolve_from_phi_recursive()` 実装
2. 循環検出ロジック実装
3. lifecycle.rs から新関数を呼び出す
4. テスト実行: 2件 → 0件 を確認

### Phase 84-5: await 暫定対応（30分）

**目標**: GroupC の 1件を解決（暫定）

**ステップ**:
1. lifecycle.rs に await 特殊ケース追加
2. テスト実行: 1件 → 0件 を確認

## 完了条件

```bash
# Phase 84-3 完了
NYASH_PHI_FALLBACK_DISABLED=1 cargo test --release --lib 2>&1 | grep "Case D" | wc -l
# 期待: 2 (GroupB のみ残存)

# Phase 84-4 完了
NYASH_PHI_FALLBACK_DISABLED=1 cargo test --release --lib 2>&1 | grep "Case D" | wc -l
# 期待: 1 (GroupC のみ残存)

# Phase 84-5 完了
NYASH_PHI_FALLBACK_DISABLED=1 cargo test --release --lib 2>&1 | grep "Case D" | wc -l
# 期待: 0 (全件解決)
```

## ChatGPT Pro との設計相談ポイント

### 相談1: Edge Copy 追跡の最適化

**質問**:
- Copy チェーンの追跡深度は 10 で十分か？
- 循環 Copy 検出は必要か？（理論上は発生しないが）
- パフォーマンス最適化（キャッシュ戦略）

### 相談2: 多段 PHI の循環検出

**質問**:
- 循環 PHI は実際に発生するか？
- 発生する場合、どう処理すべきか？（エラー or Unknown）
- visited セットの最適なデータ構造

### 相談3: await 型推論の長期戦略

**質問**:
- Phase 67+ async/await システムの型推論設計
- Safepoint/Checkpoint 命令の型情報統合方法
- 現在の暫定対応が将来の実装を妨げないか

## まとめ

Phase 84-2 の CopyTypePropagator により 12件 → 9件に削減成功。
残り 9件は以下の 3パターンに分類:

- **GroupA**: Loop 制御フロー PHI（7件）→ Edge Copy 追跡で解決可能
- **GroupB**: 多段 PHI（2件）→ 再帰的型推論で解決可能
- **GroupC**: await 特殊（1件）→ 暫定対応で解決可能

Phase 84-3/4/5 の実装により、**Case D を完全解決** できる見込み。
Status: Historical
