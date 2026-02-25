---
Status: Ready
Scope: Add `BranchN` to CorePlan (scaffold only; unconnected)
Related:
- docs/development/current/main/phases/phase-29at/README.md
- docs/development/current/main/design/match-branchn-skeleton-ssot.md
- docs/development/current/main/design/coreplan-flowbox-interface-ssot.md
- docs/development/current/main/design/post-phi-final-form-ssot.md
---

# Phase 29at P1: CorePlan `BranchN` scaffold (types + verify)

## Objective

Introduce a **first-class** `BranchN` node in `CorePlan` so `match` has a clean
final form, without changing runtime behavior yet.

This is a scaffold phase:
- `BranchN` is **not generated** by planner/normalizer yet.
- lowerer may `Err` if it ever sees `BranchN` (fail-fast).

## Non-goals

- No behavior change.
- No new env vars.
- No routing by string/pattern-name.
- No new fixtures required in P1.

## Step 1: Add CorePlan vocabulary (types only)

Target: `src/mir/builder/control_flow/plan/mod.rs`

- Add a new variant:
  - `CorePlan::BranchN(CoreBranchNPlan)`
- Add a plan struct with minimal stable shape:
  - `arms: Vec<CoreBranchArmPlan>` (>= 2)
  - `else_plans: Option<Vec<CorePlan>>`
- Each arm carries a **ValueId boolean condition** and the plans for that arm:
  - `condition: ValueId`
  - `plans: Vec<CorePlan>`

Notes:
- This matches the FlowBox ports concept: N branch arms + optional else.
- Join/payload stays in Frag/block_params, not inside CorePlan.

## Step 2: Verifier invariants (strict and local)

Target: `src/mir/builder/control_flow/plan/verifier.rs`

- Extend `PlanVerifier::verify_plan` to handle `CorePlan::BranchN`.
- Add minimal invariants:
  - arms >= 2
  - each arm has at least 1 plan (empty arm is invalid)
  - `Exit` placement follows existing rules (Seq/If behavior):
    - `Exit` only allowed as the last plan in any arm / else list
  - loop-body restrictions remain unchanged (effect-only, except allowed control
    nodes already defined by verifier policy)

Add unit tests that pin:
- OK: 2-arm BranchN with `Exit` only in last position
- NG: 1-arm BranchN (invalid)
- NG: arm with non-last `Exit` (invalid)

## Step 3: Lowerer fail-fast hook

Target: `src/mir/builder/control_flow/plan/lowerer.rs`

- Extend `PlanLowerer::lower` to match `CorePlan::BranchN(_)` and return:
  - `Err("[lowerer] CorePlan::BranchN is not yet supported".to_string())`

This is a safety net until P2/P3 introduces real lowering.

## Acceptance

- `cargo build --release`
- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

