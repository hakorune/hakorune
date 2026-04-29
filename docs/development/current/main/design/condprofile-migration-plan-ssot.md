---
Status: SSOT
Scope: CondProfile migration (ConditionShape shrink plan)
Related:
- docs/development/current/main/design/condprofile-ssot.md
- docs/development/current/main/design/condition-observation-ssot.md
- docs/development/current/main/design/recipe-first-entry-contract-ssot.md
---

# CondProfile Migration Plan SSOT

## Purpose

Shrink ConditionShape by moving condition differences into CondProfile.
This is a **structure-only** migration. No behavior changes in early phases.

## Principles (SSOT)

- BoxShape only. Do **not** mix with BoxCount.
- AST rewrite is forbidden (analysis-only views only).
- Verifier is the only acceptance gate.
- Default behavior must stay unchanged until Phase C20-D.

## Phases

### C20-A: Design lock (docs only)
- Fix the migration order and responsibilities in SSOT.
- No code changes.

### C20-B: Observation wiring
- Keep ConditionShape but add a parallel CondProfile in Facts.
- Provide a unified observation record (ConditionShape + StepShape + CondProfile).
- No routing or lowering changes.

### C20-C: Verifier priority (no behavior change)
- Verifier reads CondProfile first, but **does not change acceptance**.
- ConditionShape remains the source of truth for acceptance.

### C20-D: Shape shrink (first target = scan shapes)
- Start shrinking ConditionShape variants into CondProfile parameters.
- Acceptance is still via Verifier; routing stays stable.
- First shrink target: `ConditionShape::VarLessLength` length/size difference →
  CondProfile (shape matching ignores `LengthMethod`).
- C20-D2: `VarLessEqualLengthMinusNeedle` minus details (needle var) →
  CondProfile (shape matching ignores needle var).
- C20-D4: `VarGreaterEqualZero` 判定は CondProfile に移す
  (shape matching ignores `ConditionShape::VarGreaterEqualZero`).
- C20-D5: idx_var の一致は CondProfile 由来（shape からは参照しない）。
- C20-D7: StepShape の k 差分は CondProfile::StepExpr に移す。

## C20-D8 (plan only): ConditionShape skeleton shrink

Goal: shrink ConditionShape to CFG skeleton only (details move to CondProfile).

Target candidates (first wave):
- `VarLessLength`
- `VarLessEqualLengthMinusNeedle`
- `VarGreaterEqualZero`

Hold conditions:
- Verifier can accept based on CondProfile + Recipe (no shape dependency).

## C20-D9 (plan only): CondProfile-first verification

Purpose:
- Verifier prioritizes CondProfile evaluation.
- Accept/reject still follows ConditionShape (no behavior change yet).

Prerequisites:
- D9 completeness observation: `[condprofile:incomplete]` == 0 for 3 consecutive gates.
- D9 step mismatch observation: `[condprofile:step_mismatch]` == 0 for 3 consecutive gates.

## C20-D11 (plan only): CondProfile-first acceptance

Purpose:
- Verifier uses CondProfile as the primary acceptance source.

Prerequisites:
- `[condprofile:priority]` is stable as `condprofile` (scan facts).
- `[condprofile:step_mismatch]` == 0 for 3 consecutive gates.
- `[condprofile:incomplete]` == 0 for 3 consecutive gates.

Failure handling (design-only):
- Keep legacy fallback (no freeze).

## C20-D12: CondProfile priority acceptance (fallback)

- Prefer CondProfile-based acceptance when it matches.
- On mismatch, fall back to legacy ConditionShape acceptance (no behavior change).

## C20-D13: Drop legacy fallback when complete

- If CondProfile is complete, do not fall back to legacy ConditionShape.
- If CondProfile is incomplete, keep legacy fallback.

## C20-D14 (plan only): Incomplete handling policy

- When CondProfile is incomplete, keep legacy fallback (no fail-fast yet).
- D15 (future): consider switching fallback → freeze after observations stabilize.

## C20-D15 (plan only): incomplete observation decision

- If `[condprofile:incomplete]` is 0, consider moving to freeze (fail-fast).
- If any incomplete appears, keep legacy fallback.
- C20-D5: DONE — idx_var 差分縮退は完了（CondProfile SSOT）。drift は strict/dev で fail-fast する。
  - Pointer: `docs/development/current/main/10-Now.md` / `docs/development/current/main/design/compiler-task-map-ssot.md`
- C20-D7 (deferred): StepShape 縮退は **mismatch 観測が十分低い**ことが条件。
  - 観測源: `[condprofile:step_mismatch]`（debug-only）。
- D12-obs: legacy fallback は **0件×1回** を条件とする（運用簡略）。

## C20-D16: Freeze on incomplete (scan facts only)

- When CondProfile is incomplete, fail-fast with freeze (planner_required/dev).
- No legacy fallback for scan facts once incomplete is detected.

## C20-D17 (plan only): Expansion targets for incomplete-freeze

- Candidates (next): loop_char_map / loop_array_join / bool_predicate_scan / accum_const_loop, LoopScan*, GenericLoopV1.
- Expansion order: scan facts → loop_char_map / loop_array_join → loop_scan → generic_loop.

## C20-D18: Apply incomplete-freeze to LoopCharMap only

- Scope: LoopCharMap only (planner_required/dev; legacy numbered route label is traceability-only).
- Other scan facts keep legacy fallback for now (D19+).

## C20-D19: Apply incomplete-freeze to LoopArrayJoin

- Scope: LoopArrayJoin only (planner_required/dev; legacy numbered route label is traceability-only).

## C20-D20: Apply incomplete-freeze to BoolPredicateScan / AccumConstLoop

- Scope: BoolPredicateScan / AccumConstLoop (planner_required/dev; legacy numbered route labels are traceability-only).

## Target Order (SSOT)

1) scan_shapes (ConditionShape / StepShape)
2) loop_char_map / loop_array_join / bool_predicate_scan / accum_const_loop facts
3) generic_loop v1 (only after scan_shapes is stabilized)

## Migration Tables (SSOT)

### Current: Facts using ConditionShape/StepShape (inventory)

| Facts file | Shapes used | Notes |
| --- | --- | --- |
| `src/mir/builder/control_flow/plan/facts/loop_builder.rs` | ConditionShape, StepShape | Entry extraction via `try_extract_*` |
| `src/mir/builder/control_flow/plan/facts/loop_condition_shape.rs` | ConditionShape | Condition extractor |
| `src/mir/builder/control_flow/plan/facts/loop_step_shape.rs` | StepShape | Step extractor |
| `src/mir/builder/control_flow/plan/facts/scan_shapes.rs` | ConditionShape, StepShape | Scan helpers (cmp/bound/step) |
| `src/mir/builder/control_flow/plan/facts/loop_types.rs` | StepShape | 291x-756 removed `LoopFacts::condition_shape`; condition shape stays extractor-local |
| `src/mir/builder/control_flow/plan/facts/loop_scan_with_init.rs` | ConditionShape, StepShape | ScanWithInit facts |
| `src/mir/builder/control_flow/plan/facts/loop_array_join_facts.rs` | ConditionShape, StepShape | LoopArrayJoin facts |
| `src/mir/builder/control_flow/plan/facts/loop_char_map_facts.rs` | ConditionShape, StepShape | LoopCharMap facts |
| `src/mir/builder/control_flow/plan/facts/nested_loop_minimal_facts.rs` | ConditionShape, StepShape | NestedLoopMinimal facts |
| `src/mir/builder/control_flow/plan/facts/bool_predicate_scan_facts.rs` | ConditionShape, StepShape | BoolPredicateScan facts |
| `src/mir/builder/control_flow/plan/facts/accum_const_loop_facts.rs` | ConditionShape, StepShape | AccumConstLoop facts |

### After migration: CondProfile as the only observation source (order)

| Order | Target | Notes |
| --- | --- | --- |
| 1 | `facts/scan_shapes.rs` | Move cmp/bound/step observation to CondProfile |
| 2 | `facts/loop_condition_shape.rs` | CondProfile replaces condition extraction |
| 3 | `facts/loop_step_shape.rs` | CondProfile replaces step extraction |
| 4 | `facts/loop_builder.rs` | Wire CondProfile in the builder entry |
| 5 | `facts/loop_scan_with_init.rs` | ScanWithInit reads CondProfile only |
| 6 | `facts/loop_char_map_facts.rs` | Facts read CondProfile only |
| 7 | `facts/loop_array_join_facts.rs` | Facts read CondProfile only |
| 8 | `facts/bool_predicate_scan_facts.rs` | Facts read CondProfile only |
| 9 | `facts/accum_const_loop_facts.rs` | Facts read CondProfile only |
| 10 | `facts/nested_loop_minimal_facts.rs` | Inner loop facts read CondProfile only |

### Drift check (SSOT)

- `rg -n "ConditionShape::|StepShape::" src/mir/builder/control_flow/plan/facts` should monotonically decrease.

## Stop Conditions

- If any phase changes runtime behavior, stop and revert to the previous phase.
- If acceptance drift is detected, freeze and document in `10-Now.md`.

## Exit Criteria

- ConditionShape variants are reduced to CFG-affecting skeletons only.
- CondProfile carries parameterized details (cmp/bound/step) instead of new enum shapes.
- Verifier accepts based on CondProfile + Recipe only.

## C20-D5 Prerequisites

- idx_var contract SSOT is complete (CondProfile::LoopVar == Facts.loop_var).

## D7 Observation Rule (SSOT)

- D7 (StepShape shrink) is paused until `[condprofile:step_mismatch]` is **0**
  for **3 consecutive** planner_required dev gates.

## Ownership Policy (SSOT)

### Principles

- Facts do **not** own `String` when the value is derivable from AST/CondProfile.
  Prefer borrow (`&str`) or a stable Id/Key if it must outlive the source.
- `Option<T>` represents **incomplete observation**, not "empty".
  `None` must be treated as incomplete and surfaced in CondProfile completeness.
- CondProfile stores only **stable identifiers** or **copyable** values when possible.
  Avoid `String` in CondProfile unless required to index Recipe/Parts.

### Exceptions (allowed ownership)

- RecipeBody / RecipeParts may own `String` when it is required for lowering.
- VerifiedRecipe may own `String` if it is part of the stable, persisted contract.
- Long-lived storage across phases may own `String` when borrow lifetimes are unsafe.

### Acceptance Criteria (clone reduction scope)

- Remove clones when the source outlives the consumer (pure observation path).
- Keep clones only when the value crosses phase boundaries or is stored in Recipe.
- Do not add new clones unless required by ownership/lifetime constraints.

## Idea: Facts.loop_var should be derived from CondProfile::LoopVar (SSOT)

- Rationale: avoid mismatched extraction paths.
- Status: design-only (blocked by lifetime/refactor scope).
- Non-goal: change ownership now.
