---
title: Short-Circuit (`&&`/`||`) + `joins` SSOT
status: accepted
decision: accepted
---

# Short-Circuit (`&&`/`||`) + `joins` SSOT

## Problem

`joins` (from `build_join_payload`) is a **2-state** model: it assumes `then_val` vs `else_val` for a single `if` merge.

However, short-circuit lowering expands `a && b` / `a || b` into nested control flow which can create **3 paths**:

```text
a && b:
  Path 1: a=true,  b=true   → then_val
  Path 2: a=true,  b=false  → else_val
  Path 3: a=false           → else_val (b is not evaluated)
```

If `joins` is naively applied at the outer level, `then_val` may not dominate all paths through the outer-then region, leading to SSA/PHI issues (e.g. undefined ValueId in PHI inputs).

## Decision (SSOT)

In the current CorePlan IR, the canonical solution for “short-circuit + joins” is:

- Use `CoreEffectPlan::Copy` to define a per-join **intermediate** value on all short-circuit paths.
- Use that intermediate as the value merged by the outer PHI (`apply_if_joins`).

This keeps `joins` as a 2-state payload while making the 3-path short-circuit expansion SSA-safe.

## Scope

- Applies only to `src/mir/builder/control_flow/plan/normalizer/cond_lowering.rs` when `joins` is non-empty.
- Does not change `build_join_payload` semantics (still 3-map diff, caller-owned).

## Non-goals

- Removing `CoreEffectPlan::Copy` today.
- Introducing new plan primitives or a new canonical CFG form.

## Future directions (optional)

If we want to remove `Copy`, we likely need a larger BoxShape change:

- A CPS/backpatch-style `lower_cond(expr, on_true, on_false)` that produces leaf-branch CFG directly, and
- A single, explicit join point for "value context" boolean lowering (`lower_cond_to_value`) where a PHI is generated exactly once.

Until that primitive/normal form is defined, `Copy` remains the SSOT for correctness and simplicity.

---

## Structural Lock: PlanBuildSession (Phase 29bq+)

### Purpose

Prevent short-circuit + joins/PHI accidents **through structure**, not just code patches.

### Design Principles

1. **JoinKey is session-local**
   - Valid only within a single `PlanBuildSession` (one plan lowering call)
   - `clone_plans_with_fresh_loops` exists, so cross-session sharing is forbidden
   - IDs are issued by `PlanBuildSession`, independent of clone/freshen

2. **Sealing enforce SSOT**
   - Enforced in `emit_and_seal()`, not in `basic_block.rs`
   - **Before** `emit_frag()`: `assert_open(from_bb)` for all from blocks
   - **After** `emit_frag()` success: `seal(from_bb)` for all from blocks
   - From blocks are auto-collected from `frag.wires[*].from` / `frag.branches[*].from`

3. **Scope limitation (this phase)**
   - Only `plan/lowerer.rs` path is locked
   - Other `emit_frag` call sites (joinir/, if_form.rs, etc.) will be migrated in later phases

### Copy deletion prerequisites

When removing `Copy` in the future, the following must be in place:

1. **block params / edge args** - PHI as join block parameters
2. **Sealing** - structural prevention of "pred added after PHI inputs fixed"
3. **JoinKey** - SSOT for join block uniqueness (now session-local)

### Short-circuit join placement

- `ShortCircuitOuter` is **not** a JoinKey - use `on_true`/`on_false` targets in cond lowering
- Joins with short-circuit use `Copy` for 3-path → 2-state guarantee
- Future Copy deletion: `CondJoin` + edge-args params (value context only)

---

## Phase 29bq+ Option 2: build_logical_shortcircuit 構造ロック

### Rule 1: rhs_join 禁止（中間 join/PHI を作らない）

- 旧構造: `rhs_true → rhs_join → merge` + `rhs_false → rhs_join → merge`（2段PHI）
- 新構造: `rhs_true → merge` + `rhs_false → merge` + `skip → merge`（1段3入力PHI）
- **禁止**: RHS評価後に中間 join ブロックを挟まない

### Rule 2: merge は consumer 側 1 回

- merge ブロックは結果を使う側で 1 回だけ作成
- 各 exit path は直接 merge へ jump

### Rule 3: merge_modified_vars_multi が N-pred 変数マージの SSOT

- **ファイル**: `src/mir/builder/phi.rs`
- **用途**: short-circuit 専用（3-exit merge）
- **呼び出し箇所**: `src/mir/builder/ops.rs::build_logical_shortcircuit` のみ
- **契約**: BTreeMap<BasicBlockId, BTreeMap<String, ValueId>> で決定的順序

### 検証コマンド

```bash
# rhs_join 消滅確認
rg "rhs_join" src/mir/builder/ops.rs  # 0 件

# 呼び出し箇所確認
rg "merge_modified_vars_multi\(" src/mir/builder  # ops.rs + phi.rs(定義) のみ

# Quick smoke
./tools/smokes/v2/run.sh --profile quick  # 154/154 PASS
```

### Related

- 構造固定テスト: `src/tests/mir_controlflow_extras.rs::shortcircuit_no_inner_join_phi`
- PHI 挿入 SSOT: `src/mir/utils/phi_helpers.rs`（insert_phi*）, `src/mir/ssot/cf_common.rs`（insert_phi_at_head_spanned）
- Merge 運用ヘルパ: `src/mir/builder/phi_merge.rs`（2-pred）, `src/mir/builder/phi.rs`（N-pred）

