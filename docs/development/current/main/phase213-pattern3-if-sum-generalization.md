# Phase 213: Pattern3 Lowerer 汎用化（if-sum minimal）

**Phase**: 213
**Date**: 2025-12-09
**Status**: 🚧 In Progress
**Prerequisite**: Phase 212.5 完了（構造ベース if 検出 + Pattern 3 routing）

---

## 🎯 Phase 213 の目的

Phase 212.5 で正しく Pattern 3 にルーティングされるようになった `phase212_if_sum_min.hako` を、JoinIR Pattern 3（If-PHI）で正しく実行できるようにする。

**問題**: 現在の Pattern 3 lowerer は **test-only PoC 実装**
- Loop condition: `i <= 5` (hardcoded)
- If condition: `i % 2 == 1` (hardcoded)
- Update logic: `sum + i` (hardcoded)

**目標**: AST-based 汎用 Pattern 3 lowerer の実装
- LoopUpdateSummary / CarrierInfo / BoolExprLowerer ベースの汎用実装
- `phase212_if_sum_min.hako` で RC=2 達成
- 既存パターン（`loop_if_phi.hako` 等）の後方互換維持

---

## 📋 現状の Pattern 3 実装の問題点

### 1. ハードコードされた条件・更新式

**Loop condition** (`loop_with_if_phi_minimal.rs`):
```rust
// Hardcoded: i <= 5
let loop_cond_value = /* ... */;
```

**If condition**:
```rust
// Hardcoded: i % 2 == 1
let if_cond = /* modulo operation */;
```

**Update expressions**:
```rust
// Hardcoded: sum = sum + i, count = count + 1
let sum_update = /* sum + i */;
let count_update = /* count + 1 */;
```

### 2. テスト専用の ValueId マッピング

```rust
const PATTERN3_K_EXIT_SUM_FINAL_ID: ValueId = ValueId(24);
const PATTERN3_K_EXIT_COUNT_FINAL_ID: ValueId = ValueId(25);
```

これらは特定のテストケース用に固定されており、異なる carrier 構成には対応できない。

### 3. 汎用性の欠如

- `phase212_if_sum_min.hako` のような実際の if-sum パターンが動かない
- Carrier 構成が変わると動作しない
- If 条件が変わると対応できない

---

## 🏗️ 新しい入力情報アーキテクチャ

### 入力: PatternPipelineContext

Phase 213 では、以下の情報を利用して汎用的な lowering を実現:

**1. LoopFeatures** (from pattern_pipeline.rs)
- `has_if`: Loop body に if 文が存在するか
- `has_if_else_phi`: PHI merge が必要な if-else か
- `carrier_count`: Carrier 変数の数

**2. CarrierInfo**
- Carrier 変数のリスト（名前、host_id、join_id）
- 各 carrier の UpdateKind（CounterLike, AccumulationLike, etc.）

**3. LoopUpdateSummary**
```rust
pub struct LoopUpdateSummary {
    pub updates: Vec<CarrierUpdateInfo>,  // 各 carrier の更新情報
}

pub struct CarrierUpdateInfo {
    pub carrier_name: String,
    pub update_kind: UpdateKind,
    pub then_expr: Option<ASTNode>,   // then branch update
    pub else_expr: Option<ASTNode>,   // else branch update
}
```

**4. BoolExprLowerer / condition_to_joinir**
- 任意の bool 条件を JoinIR に変換
- 既存の `condition_to_joinir()` 関数を活用

**5. ConditionEnv / JoinValueSpace**
- Variable → ValueId マッピング
- ValueId allocation 管理

---

## 🔄 目標となる変換フロー

### Phase 213 汎用 Lowering Pipeline

```
Input: PatternPipelineContext
  ├─ loop_condition: ASTNode (e.g., "i < 3")
  ├─ loop_body: Vec<ASTNode> (contains if statement)
  ├─ CarrierInfo (e.g., [i, sum])
  └─ LoopUpdateSummary (e.g., sum: then=sum+1, else=sum+0)

Step 1: Loop Condition Lowering
  loop_condition AST → BoolExprLowerer
    → JoinIR loop_cond: ValueId

Step 2: Extract If Statement from Loop Body
  Find ASTNode::If in loop_body
    → if_condition: ASTNode (e.g., "i > 0")
    → then_body: Vec<ASTNode>
    → else_body: Option<Vec<ASTNode>>

Step 3: If Condition Lowering
  if_condition AST → BoolExprLowerer
    → JoinIR if_cond: ValueId

Step 4: Carrier Update Lowering (from LoopUpdateSummary)
  For each carrier in CarrierInfo:
    - Get then_expr from LoopUpdateSummary
    - Get else_expr from LoopUpdateSummary
    - Lower then_expr → JoinIR then_value: ValueId
    - Lower else_expr → JoinIR else_value: ValueId
    - Generate PHI: carrier_new = phi [then_value, else_value]

Step 5: JoinIR Function Generation
  - entry(): Initialize carriers
  - loop_step(i, carrier1, carrier2, ...):
      if if_cond:
        then_branch → update carriers (then values)
      else:
        else_branch → update carriers (else values)
      PHI merge → carrier_new values
      next iteration or exit
  - k_exit(carrier1_final, carrier2_final, ...): Return final values

Step 6: ExitMeta Construction
  ExitMeta {
    carriers: [
      { name: "sum", join_id: ValueId(X), host_slot: ValueId(Y) },
      ...
    ]
  }

Output: (JoinModule, ExitMeta)
```

---

## 🚨 Fail-Fast ポリシー

### 対応外パターンの明示的エラー

Pattern 3 lowerer は以下の場合に **明示的にエラー**を返す:

**1. LoopUpdateSummary 不整合**
```rust
if carrier.then_expr.is_none() || carrier.else_expr.is_none() {
    return Err(JoinIrError::UnsupportedPattern {
        reason: format!("Carrier '{}' missing then/else update", carrier.name)
    });
}
```

**2. UpdateKind 未対応**
```rust
match carrier.update_kind {
    UpdateKind::Complex | UpdateKind::Unknown => {
        return Err(JoinIrError::UnsupportedPattern {
            reason: format!("Carrier '{}' has unsupported UpdateKind: {:?}",
                           carrier.name, carrier.update_kind)
        });
    }
    _ => { /* OK */ }
}
```

**3. If 構造不整合**
```rust
if loop_body.iter().filter(|n| matches!(n, ASTNode::If { .. })).count() != 1 {
    return Err(JoinIrError::UnsupportedPattern {
        reason: "Pattern 3 requires exactly one if statement in loop body".to_string()
    });
}
```

**禁止事項**:
- ❌ Silent fallback to Pattern 1
- ❌ Default values for missing updates
- ❌ Ignoring unsupported UpdateKind

**原則**: **すべての制約は明示的エラーで通知**（Fail-Fast）

---

## 📐 設計の核心アイデア

### 1. 入力を「箱」として分離

**現状**: ハードコードされた値が scattered
**Phase 213**: 入力情報を構造化された箱から取得

```rust
// Before (Phase 195)
const LOOP_BOUND: i64 = 5;  // Hardcoded
const IF_MODULO: i64 = 2;   // Hardcoded

// After (Phase 213)
let loop_cond = ctx.loop_condition;  // From PatternPipelineContext
let if_cond = extract_if_condition(&ctx.loop_body)?;  // From AST
let updates = ctx.loop_update_summary;  // From LoopUpdateSummary
```

### 2. Lowering を既存箱に委譲

**BoolExprLowerer**: Bool condition → JoinIR
```rust
let loop_cond_value = condition_to_joinir(
    loop_cond,
    &condition_env,
    &mut join_value_space
)?;
```

**CarrierUpdateEmitter**: Update expression → JoinIR
```rust
let then_value = emit_carrier_update_with_env(
    carrier.then_expr,
    &update_env,
    &mut join_value_space
)?;
```

### 3. ExitMeta で複数 Carrier を統一的に扱う

**現状**: 固定 ValueId の const 定義
**Phase 213**: ExitMeta に動的登録

```rust
// Before
exit_bindings.push(LoopExitBinding {
    carrier_name: "sum".to_string(),
    join_exit_value: PATTERN3_K_EXIT_SUM_FINAL_ID,  // Hardcoded!
    host_slot: sum_var_id,
});

// After
for carrier in carrier_info.carriers.iter() {
    exit_bindings.push(LoopExitBinding {
        carrier_name: carrier.name.clone(),
        join_exit_value: carrier.join_final_id,  // From JoinIR generation
        host_slot: carrier.host_id,
    });
}
```

---

## 🔧 実装の構造

### Target Files

**1. JoinIR Lowerer**
- `src/mir/join_ir/lowering/loop_with_if_phi_minimal.rs`
- **変更内容**:
  - ハードコード削除
  - PatternPipelineContext からの入力受け取り
  - BoolExprLowerer / CarrierUpdateEmitter への委譲

**2. Pattern 3 Entry Point**
- `src/mir/builder/control_flow/joinir/patterns/pattern3_with_if_phi.rs`
- **変更内容**:
  - PatternPipelineContext の構築
  - ExitMeta の動的構築
  - Fail-Fast エラーハンドリング

### Signature Changes

**Before (Phase 195)**:
```rust
pub fn lower_loop_with_if_phi_pattern(
    scope: LoopScopeShape,
    join_value_space: &mut JoinValueSpace,
) -> Option<JoinModule>
```

**After (Phase 213)**:
```rust
pub fn lower_loop_with_if_phi_pattern(
    ctx: &PatternPipelineContext,
    join_value_space: &mut JoinValueSpace,
) -> Result<(JoinModule, ExitMeta), JoinIrError>
```

**変更点**:
1. 入力: `LoopScopeShape` → `PatternPipelineContext`
2. 戻り値: `Option<JoinModule>` → `Result<(JoinModule, ExitMeta), JoinIrError>`
3. ExitMeta を返して動的 exit binding を可能に

---

## ✅ 検証計画

### Test Case 1: phase212_if_sum_min.hako（主目標）

**Input**:
```nyash
loop(i < 3) {
    if i > 0 {
        sum = sum + 1
    }
    i = i + 1
}
```

**Expected**:
- RC: **2**
- Pattern: Pattern 3 (If-Else PHI)
- Carriers: `i` (CounterLike), `sum` (AccumulationLike)
- Trace: `[joinir/pattern3] Generated JoinIR for Loop with If-Else PHI`

### Test Case 2: loop_if_phi.hako（後方互換）

既存の Phase 195 テストケース:
```nyash
loop(i < 5) {
    if i % 2 == 1 {
        sum = sum + i
    } else {
        sum = sum + 0
    }
    i = i + 1
}
```

**Expected**:
- 既存と同じ出力・RC
- Regression なし

### Test Case 3: Multi-carrier Phase 195 tests

Phase 195 で追加された multi-carrier tests:
- sum + count の 2-carrier
- sum + count + index の 3-carrier (if exists)

**Expected**:
- 既存と同じ挙動
- ExitMeta が複数 carrier を正しく処理

---

## 📊 Phase 213 タスクチェックリスト

- [ ] Task 213-1: 設計ドキュメント作成 ✅ (this file)
- [ ] Task 213-2: Pattern3 Lowerer 本体リファクタリング
  - [ ] Step 2-1: ハードコード削除
  - [ ] Step 2-2: 入力を Context ベースに変更
  - [ ] Step 2-3: 条件 lowering を BoolExprLowerer に委譲
  - [ ] Step 2-4: キャリア更新の一般化
  - [ ] Step 2-5: PHI 生成
  - [ ] Step 2-6: 戻り値と boundary 連携
- [ ] Task 213-3: Fail-Fast 条件の明確化
- [ ] Task 213-4: テスト & 検証
  - [ ] phase212_if_sum_min.hako → RC=2
  - [ ] loop_if_phi.hako → Regression check
  - [ ] Multi-carrier tests → Regression check
- [ ] Task 213-5: ドキュメント更新
  - [ ] phase212-if-sum-impl.md
  - [ ] joinir-architecture-overview.md
  - [ ] CURRENT_TASK.md

---

## 🎯 Success Criteria

**Phase 213 is complete when**:
1. ✅ `phase212_if_sum_min.hako` produces RC=2
2. ✅ All existing Pattern 3 tests pass (no regression)
3. ✅ No hardcoded conditions/updates in Pattern 3 lowerer
4. ✅ Fail-Fast errors for unsupported patterns
5. ✅ Documentation updated (3 files)

**Commit message format**:
```
feat(joinir): Phase 213 Pattern3 AST-based generalization

Phase 213 で Pattern3 lowerer を AST-based 汎用実装に書き換え。
phase212_if_sum_min.hako が RC=2 で正常動作。

- Removed hardcoded conditions/updates
- Integrated BoolExprLowerer for dynamic condition lowering
- Generalized carrier update via LoopUpdateSummary
- Dynamic ExitMeta construction for multi-carrier support
- Fail-Fast for unsupported patterns
```

---

**Phase 213: READY TO START** 🚀
Status: Active  
Scope: Pattern3 If-sum の一般化（JoinIR v2）
