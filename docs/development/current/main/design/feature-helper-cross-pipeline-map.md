---
Status: SSOT (navigation)
Scope: plan/features の helper 境界と “どこで何を作るか” の地図（cross-pipeline）
Related:
- docs/development/current/main/design/feature-helper-boundary-ssot.md
- docs/development/current/main/design/compiler-cleanliness-campaign-ssot.md
- docs/development/current/main/design/coreplan-skeleton-feature-model.md
- docs/development/current/main/design/planner-entry-guards-ssot.md
- src/mir/builder/control_flow/plan/REGISTRY.md
---

# Feature helper cross-pipeline map (SSOT)

目的: pipeline/ops が迷走しないように、「何をどこで作るか」を 1 枚で辿れる導線にする。
（selfhost を目的化しない。BoxShape-first の作業を継続できるようにする。）

## High-level flow（責務の矢印）

1. **Facts**: 受理形の観測（analysis-only view を使う。AST rewrite しない）
2. **Skeleton**: ブロック骨格（frame）と slot を用意
3. **Features / steps**: slot に部品を適用し、CorePlan を組む
4. **Lower / Emitter**: CorePlan → MIR/JoinIR

## SSOT helpers（“作り場”）

pipeline/ops は下記 SSOT helper を呼ぶだけに寄せる（直書きを増やさない）。

- **Reject/Freeze + handoff**
  - `src/mir/builder/control_flow/plan/facts/reject_reason.rs`
  - ログ: `[plan/reject]`, `[plan/accept]`, `[plan/trace]`（契約は `planner-entry-guards-ssot.md`）

- **Exit plans**
  - `src/mir/builder/control_flow/plan/features/exit_branch.rs`
  - `CoreExitPlan::{Break,Continue,Return,ContinueWithPhiArgs}` の生成はここへ寄せる

- **EdgeCFG stubs（wires/branches）**
  - `src/mir/builder/control_flow/plan/features/edgecfg_stubs.rs`
  - `BranchStub` / `EdgeStub` の直書きを features/** に入れない

- **Carrier sets（vars/ids の入口）**
  - `src/mir/builder/control_flow/plan/features/carriers.rs`
  - “どの vars を carrier として扱うか” の入口を固定する

- **Loop PHI / bindings / EdgeArgs**
  - `src/mir/builder/control_flow/plan/features/loop_carriers.rs`
  - `CorePhiInfo` / `create_phi_bindings` の直書きは `loop_carriers` のみに制限する
  - 将来の設計: `docs/development/current/main/design/phi-input-strategy-ssot.md`

- **StepMode setters**
  - `src/mir/builder/control_flow/plan/step_mode.rs`（plan-wide SSOT）
  - `src/mir/builder/control_flow/plan/features/step_mode.rs`（features 側 adapter）
  - `LoopStepMode::*` と `has_explicit_step` の組は helper で固定する

- **Steps（固定順の工程部品）**
  - `src/mir/builder/control_flow/plan/features/steps/`
  - “同型が2箇所以上” のときだけ steps 化する（Body lowering inventory は `feature-helper-boundary-ssot.md`）

## Quick triage（迷ったらここ）

- `BranchStub { ... }` / `EdgeStub { ... }` を書きたくなった → `edgecfg_stubs.rs`
- `CoreExitPlan::...` を組みたくなった → `exit_branch.rs`
- PHI / bindings を直書きしたくなった → `loop_carriers.rs`
- `step_mode` を設定したくなった → `step_mode.rs`
- reject/freeze 文字列を増やしたくなった → `RejectReason::as_freeze_message()`（直書き禁止）
- carrier vars の入口が増えそう → `carriers.rs` の SSOT entry を増やす（1個だけ）

## Closeout inventories（回帰防止）

"逸脱が 0" を確認するための棚卸しコマンドは `feature-helper-boundary-ssot.md` の checklist を参照する。

## Recipe Shape Inventory

Shape types (`recipes/refs.rs`):
- `StmtRef { idx: StmtIdx }` - single statement reference
- `StmtPair { a: StmtIdx, b: StmtIdx }` - pair of statements
- `StmtSpan { range: StmtRange }` - span of statements (shape-only; interpretation is box-local)

Notes:
- `recipes/refs.rs` is **shape-only** (meaning-neutral). Semantic vocabulary stays in each box’s recipe enum.
- `StmtSpan` is body-local; `start` is not required to be 0. Verification only checks `start <= end` and in-bounds.
- `StmtRef` is **body-local by default**. If a recipe explicitly carries branch-local refs (then/else),
  verification must use the **branch length**, not the loop-body length.
- Recipe-first boxes must follow: Facts builds Recipe → pipeline lowers **recipe items only** (no re-validation).
- RecipeBody policy:
  - A box that adopts recipe-first **must own** its original body as `recipes::RecipeBody`.
  - Non recipe-first boxes are **not required** to adopt `RecipeBody` (avoid broad forced migrations).
- Accessor policy (readability lock):
  - Prefer shape-only accessors (e.g. `StmtRef::index()` / `RecipeBody::get_ref(StmtRef)`) over `idx.0` direct field access in pipelines/recipes.

| Box | Variant | Shape | Acceptance Notes |
|-----|---------|-------|------------------|
| loop_true_break_continue | Stmt | StmtRef | simple stmt only (no If/Loop) |
| loop_true_break_continue | ProgramGeneralBlock | StmtRef | Program with general-if body |
| loop_true_break_continue | ExitIf | StmtRef | break/continue/return(value) |
| loop_true_break_continue | IfTailExitPair | StmtPair | if + tail exit pair |
| loop_true_break_continue | NestedLoopDepth1 | StmtRef | nested loop depth=1 |
| loop_true_break_continue | GeneralIf | StmtRef | carrier update only, no exit |
| loop_cond_continue_only | Stmt | StmtRef | normal stmt (lower_stmt_ast) |
| loop_cond_continue_only | ContinueIf | StmtSpan | `if <cond> { <prelude>; continue }` (prelude is a span inside then_body; tail-continue excluded) |
| loop_cond_continue_only | ContinueIfGroupPrelude | StmtSpan | nested continue-if/group-if inside prelude (items describe nested structure) |
| loop_cond_continue_only | GroupIf | StmtRef | grouping if containing ContinueIf recipes |
| loop_cond_continue_only | ContinueIfNestedLoop | StmtSpan | nested loop(depth=1) inside continue-if (prelude/postlude span+items; fixture-derived) |
| loop_cond_break_continue | Stmt | StmtRef | normal stmt (assignment/local/call/print) |
| loop_cond_break_continue | IfAny | StmtRef | any accepted if (lowered by pipeline ordering) |
| loop_cond_break_continue | NestedLoopDepth1 | StmtRef | nested loop depth=1 allowed by facts |
| loop_cond_break_continue | ProgramBlock | (recipe) | Program lowered as loop-cond sub-block (no exits) |
| loop_cond_break_continue | TailBreak | (none) | tail break at top level |
| loop_cond_break_continue | ElseOnlyReturnIf | StmtRef | `else_return_stmt` is else-branch local index |
| loop_cond_break_continue | ThenOnlyReturnIf | StmtRef | `then_return_stmt` is then-branch local index |
| loop_cond_break_continue | ElseOnlyBreakIf | StmtRef | `else_break_stmt` is else-branch local index |
| loop_cond_break_continue | ThenOnlyBreakIf | StmtRef | `then_break_stmt` is then-branch local index |
| loop_cond_return_in_body | Stmt | StmtRef | fixture-derived 1-shape; pipeline lowers sequential stmt refs |
| loop_cond_continue_with_return | Stmt | StmtRef | normal stmt (incl. assignments/calls) |
| loop_cond_continue_with_return | ContinueIf | StmtSpan | `if <cond> { <prelude>; continue }` (prelude is a span inside then_body; items describe nested structure) |
| loop_cond_continue_with_return | HeteroReturnIf | StmtRef | fixture-derived hetero if-else-if chain with nested return |
| loop_cond_continue_with_return | IfAny | StmtRef | any other accepted if (pipeline ordering) |
