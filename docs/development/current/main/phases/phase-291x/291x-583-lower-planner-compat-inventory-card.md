---
Status: Landed
Date: 2026-04-28
Scope: inventory lower::planner_compat and prune zero-use re-exports while keeping the live lowering boundary
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-575-plan-compat-residue-inventory-card.md
  - src/mir/builder/control_flow/lower/mod.rs
  - src/mir/builder/control_flow/lower/planner_compat.rs
  - src/mir/builder/control_flow/plan/lowerer.rs
---

# 291x-583: Lower Planner Compat Inventory

## Goal

Inventory `lower::planner_compat`, confirm whether it is a pure facade or a
live owner boundary, and land the smallest safe cleanup slice.

This stays BoxShape-only. It does not move `PlanLowerer` ownership or change
planner/lowering behavior.

## Evidence

The inventory showed that `lower/planner_compat.rs` is a pure re-export facade
over plan-owned symbols, but not all of its exports are still live.

The following re-exports had no non-plan callers and were only compatibility
surface residue:

- `build_plan_with_facts`
- `build_plan_with_facts_ctx`
- `PlannerContext`
- `planner_rule_semantic_label`
- `planner_rule_tag_name`

The remaining surface is still live and forms the lower-side boundary used by
JoinIR routing, verifier code, and return lowering.

## Boundaries

- Prune only zero-use re-exports from `lower/planner_compat.rs` and
  `lower/mod.rs`.
- Keep `PlanLowerer`, `PlanBuildOutcome`, `Freeze`, `CorePlan`, and related live
  boundary types in place.
- Do not move implementations out of `plan/`.

## Acceptance

- No non-plan callers depend on the five removed re-exports.
- `lower::planner_compat` keeps the still-live lowering boundary intact.
- `bash tools/checks/current_state_pointer_guard.sh` passes.
- `cargo check --release --bin hakorune` passes.
- `cargo fmt -- --check` passes.
- `git diff --check` passes.

## Result

- Confirmed `lower::planner_compat` is a pure facade, not an implementation
  owner.
- Removed five zero-use re-exports from the facade and the top-level `lower`
  surface.
- Left the live lower/planner boundary in place for a future design card if
  physical ownership should move.

## Verification

```bash
rg -n "build_plan_with_facts|build_plan_with_facts_ctx|PlannerContext|planner_rule_semantic_label|planner_rule_tag_name" src tests -g'*.rs'
bash tools/checks/current_state_pointer_guard.sh
cargo fmt -- --check
cargo check --release --bin hakorune
git diff --check
```
