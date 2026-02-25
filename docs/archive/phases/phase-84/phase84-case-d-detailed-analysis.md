# Phase 84: Case D 詳細分析レポート

## 概要

Phase 83 で Case D が 20 件 → **15 件** に減少（MethodReturnHintBox 実装）。  
その後 Phase 84-1（Const 命令型アノテーション追加）で **12 件**、Phase 84-2（CopyTypePropagator 導入）で **9 件** まで削減された。

本レポート自体は「24 件あった調査時点」の分析ログとして残しつつ、  
現在は Const 欠如グループと単純な Copy チェーンは解消され、残りは主に PHI を含む複雑なパターンであることが判明している。

**重要な発見（当時）**: 主要な原因は **Const命令の型アノテーション欠如** である。  
**補足（現在）**: Const 命令については 40dfbc68 で修正済み、Copy 伝播については CopyTypePropagator（Phase 84-2）で整理済み。

---

## 実行環境

```bash
NYASH_PHI_FALLBACK_DISABLED=1 cargo test --release --lib 2>&1
```

**テスト結果（当時）**: 471 passed; **52 failed** (うち Case D は24件)

---

## Case D 失敗一覧（24件）

| # | Test Name | Function | ValueId | 推測される原因 |
|---|-----------|----------|---------|--------------|
| 1 | test_lowering_await_expression | main | ValueId(2) | Await式の戻り値型 |
| 2 | test_try_catch_compilation | main | ValueId(7) | Try-Catch式の戻り値型 |
| 3 | loop_break_and_early_return_edge_copy | main | ValueId(50) | Loop exit後の型 |
| 4 | loop_with_continue_and_break_edge_copy_merge | main | ValueId(56) | Loop exit後の型 |
| 5 | vm_exec_break_inside_if | main | ValueId(27) | Loop break後の型 |
| 6 | **mir_locals_uninitialized** | main | **ValueId(1)** | **return 0 の型** |
| 7 | loop_inner_if_multilevel_edge_copy | main | ValueId(74) | Loop exit後の型 |
| 8 | nested_loop_with_multi_continue_break_edge_copy_merge | main | ValueId(135) | Loop exit後の型 |
| 9 | mir13_no_phi_if_merge_inserts_edge_copies_for_return | main | ValueId(17) | If merge後の型 |
| 10 | loop_if_three_level_merge_edge_copy | main | ValueId(75) | Loop exit後の型 |
| 11 | nested_if_inside_loop_edges_copy_from_exiting_blocks | main | ValueId(6) | Loop exit後の型 |
| 12 | nested_loops_break_continue_mixed | main | ValueId(8) | Loop exit後の型 |
| 13 | mir_funcscanner_skip_ws_min_verify_and_vm | main | ValueId(76) | 複雑な制御フロー |
| 14-20 | mir_stageb_like_*_verifies | main | ValueId(1) | **return 系の型** |
| 21 | mir_stage1_cli_emit_program_min_compiles_and_verifies | main | ValueId(7) | Stage-1パターン |
| 22 | mir_stage1_cli_emit_program_min_exec_hits_type_error | main | ValueId(7) | Stage-1パターン |
| 23 | mir_jsonscanbox_like_seek_array_end_verifies | main | ValueId(2) | 複雑なメソッド |
| 24 | mir_stage1_cli_entry_like_pattern_verifies | main | ValueId(1) | **return 系の型** |

---

## パターン別分類

### GroupA: **Const命令型アノテーション欠如**（推定 14-16件）

#### 根本原因

`src/mir/builder/emission/constant.rs` で、**Integer/Bool/Float/Null/Void 定数は `value_types` に登録されない**:

```rust
// ❌ 型アノテーションなし
pub fn emit_integer(b: &mut MirBuilder, val: i64) -> ValueId {
    let dst = b.next_value_id();
    let _ = b.emit_instruction(MirInstruction::Const {
        dst,
        value: ConstValue::Integer(val),
    });
    dst  // ← value_types に何も登録していない！
}

// ✅ String のみ型アノテーションあり
pub fn emit_string<S: Into<String>>(b: &mut MirBuilder, s: S) -> ValueId {
    let dst = b.next_value_id();
    let _ = b.emit_instruction(MirInstruction::Const {
        dst,
        value: ConstValue::String(s.into()),
    });
    // 🎯 Phase 3-A: String constant type annotation
    b.value_types.insert(dst, MirType::Box("StringBox".to_string()));
    b.value_origin_newbox.insert(dst, "StringBox".to_string());
    dst
}
```

#### 影響範囲

- `return 0` のような整数リテラル return
- `return true`/`return false` のような真偽値 return
- `return 3.14` のような浮動小数点 return
- `return null` や `return void`

#### 該当テスト

- #6: mir_locals_uninitialized (`return 0`)
- #14-20: mir_stageb_like_*_verifies (全て `return` 系)
- #24: mir_stage1_cli_entry_like_pattern_verifies

---

### GroupB: **Copy命令型伝播不足**（推定 6-8件）

#### 根本原因

`Copy` 命令で値がコピーされた際、型情報が伝播しないケースがある。

`src/mir/builder/metadata/propagate.rs` には型伝播機能があるが、**すべての Copy 命令で呼ばれるわけではない**:

```rust
// Phase 3: 型伝播（一部のケースのみ）
if let Some(t) = builder.value_types.get(&src).cloned() {
    builder.value_types.insert(dst, t);
}
```

#### 影響範囲

- Loop exit 後の edge copy
- If merge 後の edge copy
- PHI 命令からの Copy

#### 該当テスト

- #3-5, #7-8, #10-12: Loop break/continue 後の edge copy
- #9: If merge 後の edge copy
- #11: Loop 内 If の edge copy

---

### GroupC: **PHI命令型推論不足**（推定 4-6件）

#### 根本原因

`GenericTypeResolver::resolve_from_phi()` は以下のケースで失敗する:

1. **ret_val が PHI の出力ではない**
2. **PHI の incoming 値の型が不一致**
3. **incoming 値の型が `value_types` に未登録**

```rust
pub fn resolve_from_phi(
    function: &MirFunction,
    ret_val: ValueId,
    types: &BTreeMap<ValueId, MirType>,
) -> Option<MirType> {
    for (_bid, bb) in function.blocks.iter() {
        for inst in bb.instructions.iter() {
            if let MirInstruction::Phi { dst, inputs, .. } = inst {
                if *dst == ret_val {
                    let mut it = inputs.iter().filter_map(|(_, v)| types.get(v));
                    if let Some(first) = it.next() {
                        if it.all(|mt| mt == first) {
                            return Some(first.clone());  // ← 全て同じ型の時のみ成功
                        }
                    }
                }
            }
        }
    }
    None  // ← PHI が見つからない、または型不一致
}
```

#### 影響範囲

- 複雑な制御フロー（Await/Try-Catch）
- 多段 PHI チェーン（PHI → Copy → PHI）
- 型が異なる incoming 値を持つ PHI

#### 該当テスト

- #1: test_lowering_await_expression (Await式)
- #2: test_try_catch_compilation (Try-Catch式)
- #13: mir_funcscanner_skip_ws_min_verify_and_vm
- #21-22: Stage-1 CLI パターン

---

### GroupD: **その他の命令型**（推定 2-4件）

#### 可能性のある原因

- **BoxCall/Call 命令の戻り値型** が未登録
- **NewBox 命令の戻り値型** が未登録（稀）
- **TypeOp 命令の戻り値型** が未登録
- **UnaryOp/BinOp 命令の戻り値型** が一部未登録

#### 該当テスト

- #23: mir_jsonscanbox_like_seek_array_end_verifies (複雑なメソッド)

---

## 解決策の優先順位

### Phase 84-1: **Const命令型アノテーション追加**（最優先・最大効果）

**期待される効果**: 14-16件のテストが修正される（58-67%）

**実装箇所**: `src/mir/builder/emission/constant.rs`

```rust
// ✅ 修正案
pub fn emit_integer(b: &mut MirBuilder, val: i64) -> ValueId {
    let dst = b.next_value_id();
    let _ = b.emit_instruction(MirInstruction::Const {
        dst,
        value: ConstValue::Integer(val),
    });
    b.value_types.insert(dst, MirType::Integer);  // ← 追加
    dst
}

pub fn emit_bool(b: &mut MirBuilder, val: bool) -> ValueId {
    let dst = b.next_value_id();
    let _ = b.emit_instruction(MirInstruction::Const {
        dst,
        value: ConstValue::Bool(val),
    });
    b.value_types.insert(dst, MirType::Bool);  // ← 追加
    dst
}

pub fn emit_float(b: &mut MirBuilder, val: f64) -> ValueId {
    let dst = b.next_value_id();
    let _ = b.emit_instruction(MirInstruction::Const {
        dst,
        value: ConstValue::Float(val),
    });
    b.value_types.insert(dst, MirType::Float);  // ← 追加
    dst
}

pub fn emit_null(b: &mut MirBuilder) -> ValueId {
    let dst = b.next_value_id();
    let _ = b.emit_instruction(MirInstruction::Const {
        dst,
        value: ConstValue::Null,
    });
    b.value_types.insert(dst, MirType::Null);  // ← 追加
    dst
}

pub fn emit_void(b: &mut MirBuilder) -> ValueId {
    let dst = b.next_value_id();
    let _ = b.emit_instruction(MirInstruction::Const {
        dst,
        value: ConstValue::Void,
    });
    b.value_types.insert(dst, MirType::Void);  // ← 追加
    dst
}
```

**リスク**: 極めて低い（String は既に実装済み）

---

### Phase 84-2: **Copy命令型伝播の徹底**（次点・中効果）

**期待される効果**: 6-8件のテストが修正される（25-33%）

**実装箇所**:
- `src/mir/builder/ssa/local.rs` - Local変数のCopy
- `src/mir/builder/metadata/propagate.rs` - 一般的な型伝播
- `src/mir/phi_core/loop_phi.rs` - Loop exit edge copy
- `src/mir/phi_core/if_phi.rs` - If merge edge copy

**実装方針**:
1. すべての `emit_copy()` 呼び出し箇所を洗い出す
2. 型伝播が欠けている箇所に `propagate_type()` を追加
3. Edge copy 生成時に元の ValueId の型を継承

```rust
// ✅ 修正案（例：loop_phi.rs）
fn emit_edge_copy(builder: &mut MirBuilder, src: ValueId) -> ValueId {
    let dst = builder.next_value_id();
    builder.emit_instruction(MirInstruction::Copy { dst, src });

    // 型伝播を追加
    if let Some(ty) = builder.value_types.get(&src).cloned() {
        builder.value_types.insert(dst, ty);
    }

    dst
}
```

**リスク**: 中程度（既存の型伝播ロジックとの整合性確認が必要）

---

### Phase 84-3: **PHI型推論の強化**（長期・小効果）

**期待される効果**: 4-6件のテストが修正される（17-25%）

**実装方針**:
1. **多段PHIチェーン対応**: PHI → Copy → PHI の解析
2. **型不一致PHI対応**: 共通の上位型（例: Integer | Null → Any）
3. **await/try-catch専用**: 特殊構文用の型ヒント

**実装箇所**: `src/mir/join_ir/lowering/generic_type_resolver.rs`

```rust
// ✅ 拡張案
pub fn resolve_from_phi_recursive(
    function: &MirFunction,
    ret_val: ValueId,
    types: &BTreeMap<ValueId, MirType>,
    depth: usize,  // 再帰深度制限
) -> Option<MirType> {
    if depth > 5 {
        return None;  // 無限ループ防止
    }

    // 1. 直接 PHI を探す
    if let Some(ty) = resolve_from_phi(function, ret_val, types) {
        return Some(ty);
    }

    // 2. Copy 経由で PHI を探す
    for (_bid, bb) in function.blocks.iter() {
        for inst in bb.instructions.iter() {
            if let MirInstruction::Copy { dst, src } = inst {
                if *dst == ret_val {
                    return resolve_from_phi_recursive(function, *src, types, depth + 1);
                }
            }
        }
    }

    None
}
```

**リスク**: 高（再帰的解析のパフォーマンスと正確性）

---

## アクションアイテム

### 即座に実装すべき項目

1. **Phase 84-1 実装** (最優先)
   - `constant.rs` の 5 関数に型アノテーション追加
   - テスト実行して 14-16 件の修正を確認
   - 期待: Case D が 24件 → 8-10件に削減

2. **Phase 84-2 実装** (次点)
   - Copy 命令の型伝播を徹底
   - Edge copy 専用のヘルパー関数を作成
   - 期待: Case D が 8-10件 → 2-4件に削減

### 後回しでも良い項目

3. **Phase 84-3 検討** (長期)
   - 多段 PHI チェーンが本当に必要か検証
   - await/try-catch の型推論を専用実装で対応
   - 期待: Case D が 2-4件 → 0件（完全解決）

---

## まとめ

**Case D の主要原因**:
1. **Const命令の型アノテーション欠如** (58-67%)
2. **Copy命令の型伝播不足** (25-33%)
3. **PHI型推論の限界** (17-25%)

**推奨アプローチ**:
- Phase 84-1 を即座に実装（1-2時間で完了、大幅改善）
- Phase 84-2 を段階的に実装（1-2日で完了、ほぼ完全解決）
- Phase 84-3 は残存ケースを見てから判断

**期待される最終結果**:
- Case D: 24件 → **0-2件**（90-100%解決）
- テスト成功率: 471/523 (90%) → **519-521/523 (99-100%)**
Status: Historical
