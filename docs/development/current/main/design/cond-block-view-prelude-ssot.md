# CondBlockView Prelude SSOT

Purpose: define the contract for “condition prelude” (statements evaluated before the final condition expression) without AST rewrite, and prevent accidental semantics drift.

This is a **design SSOT** (no behavior change by itself).

## Current State (SSOT)

- `CondBlockView` exists as an analysis-only view: `src/mir/builder/control_flow/facts/canon/cond_block_view.rs`
  - Phase B2: `CondBlockView::from_expr(...)` extracts `ASTNode::BlockExpr { prelude_stmts, tail_expr }` directly.
  - Therefore, Rust AST **can now represent** condition prelude *structurally* via BlockExpr.
- Current JoinIR/plan contract (dev/strict only, fail-fast):
  - `CondBlockView` prelude is **supported** at plan condition entry (Phase B4).
    - Lowering SSOT: `src/mir/builder/control_flow/plan/normalizer/cond_lowering.rs`
      - `lower_cond_branch(...)` / `lower_cond_value(...)` / `lower_loop_header_cond(...)`
      - Prelude statements are lowered to `CoreEffectPlan` in source order and inserted before tail condition lowering.
    - Facts SSOT: `src/mir/builder/control_flow/plan/facts/expr/bool_expr.rs`
      - BlockExpr-in-condition is treated as "tail bool expr" gated by prelude vocabulary + exit-free contract.

## Why This Contract Exists

We do not want to silently accept “prelude-like” patterns unless the language AST and lowering contract explicitly support them.

- No AST rewrite allowed (policy).
- Without an explicit AST representation, any “prelude extraction” would be heuristic and risk semantics drift.

## B2 Outcome: BlockExpr Achieves Condition Prelude (SSOT)

**With Phase B2 complete**, the "Alternative path (preferred)" is now the actual path:

- Map literals moved to `%{...}` (Phase B1)
- `{...}` is now a block expression (Phase B2)
- Condition prelude is expressed as `if ({ prelude; cond_expr }) { ... }`

**Decision: No `cond { ... }` syntax needed.**

BlockExpr in condition position achieves the condition prelude goal without adding new syntax:

```nyash
if ({
    local x = compute()
    x > 0
}) {
    // use x
}
```

v1 constraints (enforced by MIR lowering):
- `tail_expr` is required (the final condition expression)
- Exit statements (`return/break/continue/throw`) are forbidden anywhere inside the BlockExpr

**Related**:
- `docs/development/current/main/design/map-literal-eviction-and-blockexpr-roadmap-ssot.md` - B2 implementation
- `docs/reference/language/block-expressions-and-map-literals.md` - BlockExpr spec

## Future Support (if needed beyond BlockExpr)

If BlockExpr proves insufficient, supporting explicit condition prelude would require:

1. **Parser/AST representation (required)**
   - Introduce a dedicated AST node that can carry:
     - `prelude_stmts: Vec<ASTNode>` (stmt-only, no exits)
     - `tail_expr: ASTNode` (evaluated once)
   - This is a language/AST change and must be accompanied by a Decision (accepted/provisional) before implementation.

   Alternative path (adopt BlockExpr):
   - ✅ Implemented via Phase B2 (BlockExpr carries prelude+tail).

2. **View extraction remains structural (no rewrite)**
   - Keep mapping 1:1 from an AST node (BlockExpr or a dedicated Cond node) into `CondBlockView`.
   - Reject unknown forms (fail-fast), no heuristic splitting.

3. **Lowering contract (SSOT)**
   - Execute `prelude_stmts` exactly once at the condition entry, then branch on `tail_expr`.
   - Preconditions (mechanical, verifier-enforced):
     - prelude is "stmt-only" (no `break/continue/return`, no nested terminators)
     - no fallthrough-after-exit inside prelude
   - `parts::dispatch` remains the SSOT for "if assembly": prelude plans + `lower_cond_branch(...)`.

## Drift Checks (SSOT)

- CondBlockView extracts BlockExpr prelude:
  - `rg -n "ASTNode::BlockExpr" src/mir/builder/control_flow/facts/canon/cond_block_view.rs` → 1件以上
- Plan-side prelude rejects are removed (Phase B4 completed):
  - `rg -n "CondBlockView prelude is not supported yet" src/mir/builder/control_flow/plan/normalizer/cond_lowering.rs` → 0件
  - `rg -n "cond_prelude_unsupported" src/mir/builder/control_flow/plan/parts/verify.rs` → 0件
  - `rg -n "CondBlockView prelude is not supported yet" src/mir/builder/control_flow/plan/normalizer/cond_lowering.rs src/mir/builder/control_flow/plan/parts/verify.rs` → 0件

- Cond prelude vocabulary is SSOT-backed (no duplication):
  - `rg -n "mod cond_prelude_vocab" src/mir/builder/control_flow/cleanup/policies/mod.rs` → 1件
  - `rg -n "classify_cond_prelude_stmt" src/mir/builder/control_flow/plan/facts/expr/bool_expr.rs src/mir/builder/control_flow/plan/normalizer/cond_lowering.rs` → 2件

- Bool-condition lowering entry is explicit (SSOT helper):
  - `rg -n "pub fn lower_bool_expr_value_id" src/mir/builder/control_flow/plan/normalizer/cond_lowering.rs` → 1件
  - `rg -n "lower_bool_expr_value_id\\(" src/mir/builder/control_flow/plan` → 1件以上

- Guardrail: bool conditions are not value-lowered (regression check for a past footgun):
  - `rg -n "lower_value_ast\\(\\s*&facts\\.recipe\\.(comma_if_cond|close_if_cond)" src/mir/builder/control_flow/plan/loop_scan_v0/pipeline.rs` → 0件

## Phase B4: Plan-side condition prelude lowering

Goal: make `if ({ prelude; cond })` and `loop({ prelude; cond })` usable under planner-required paths (CorePlan/Parts) without widening accept shapes.

- SSOT entry: `src/mir/builder/control_flow/plan/normalizer/cond_lowering.rs`
- Contract (v1):
  - prelude statements are lowered to `CoreEffectPlan` in source order (no rewrite)
  - `tail_expr` is evaluated once
  - exit is forbidden anywhere inside prelude (reuse BlockExpr contract)
  - initial scope policy is “compiler-local” (no new lexical-scope feature); later tightening is a separate decision

Status: ✅ Completed

Pinned acceptance:
- `apps/tests/phase29bq_cond_prelude_planner_required_min.hako` (fast gate pinned)
