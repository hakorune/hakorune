---
Status: Ready
Scope: Implement CorePlan::BranchN lowering (behavior-neutral; not generated yet)
Related:
- docs/development/current/main/phases/phase-29at/README.md
- docs/development/current/main/design/match-branchn-skeleton-ssot.md
- docs/development/current/main/design/coreplan-flowbox-interface-ssot.md
- docs/development/current/main/design/post-phi-final-form-ssot.md
---

# Phase 29at P2: CorePlan::BranchN lowering (minimal)

## Objective

Make `CorePlan::BranchN` **lowerable** to MIR so future `match → BranchN`
generation can land without a large follow-up.

This phase is behavior-neutral:
- Planner/Normalizer still do not create `BranchN` in release/strict/dev.
- We only add a lowering implementation + local verifier tests.

## Non-goals

- No `match` support in planner/normalizer yet (that is P3+).
- No new env vars.
- No by-name routing.
- No changes to runtime/stdlib behavior.

## Implementation outline

### Step 1: Lowerer support via “If-chain rewrite”

Target: `src/mir/builder/control_flow/plan/lowerer.rs`

- Implement `CorePlan::BranchN` lowering by translating it into a nested `CorePlan::If`
  chain and reusing the existing `lower_if` path.
- The transformation is internal-only:
  - `BranchN` stays the SSOT skeleton at the plan layer.
  - MIR generation uses ordinary branches/jumps as usual.

Recommended conversion (sketch):

- Convert `BranchN { arms: [a0, a1, ...], else_plans }` into:
  - `If(cond=a0.cond, then=a0.plans, else=[If(cond=a1.cond, then=a1.plans, else=[...])])`
  - Final else is `else_plans` (or `None` meaning fallthrough/no-op).

### Step 2: Keep verifier invariants unchanged

Target: `src/mir/builder/control_flow/plan/verifier.rs`

- Do not weaken P1 invariants.
- Ensure the same “Exit is last” rule applies inside each arm list.

### Step 3: Unit test for lowering “does not fail-fast”

Target: `src/mir/builder/control_flow/plan/lowerer.rs`

- Add a small unit test that constructs a `CorePlan::BranchN` whose arms
  contain only Effects (or Effects + final Exit) and asserts:
  - `PlanLowerer::lower(...)` does not return the old “not yet supported” error.

The test can remain minimal (no need to assert exact MIR layout).

## Acceptance (SSOT)

- `cargo build --release`
- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

