---
Status: Landed
Date: 2026-04-28
Scope: prune lower planner compat test-only exports
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/mir/builder/control_flow/lower/mod.rs
  - src/mir/builder/control_flow/lower/planner_compat.rs
  - src/mir/builder/control_flow/verify/verifier/tests.rs
---

# 291x-658: Lower Planner Compat Test Export Prune

## Goal

Remove lower-facade exports that are only used by verifier tests.

This is BoxShape cleanup. It must not change planner ownership, lowering
behavior, route selection, verifier behavior, or `PlanLowerer` placement.

## Evidence

Worker inventory found no clear zero-use exports left in
`lower/planner_compat.rs`, but identified two lower-facade imports used only by
`verify/verifier/tests.rs`:

- `CoreBranchArmPlan`
- `CoreIfJoin`

Both are plan-owned recipe/shape types. The verifier tests can import them from
the plan owner directly instead of keeping them alive through the lower facade.

## Decision

Make verifier tests import `CoreBranchArmPlan` and `CoreIfJoin` from
`control_flow::plan`.

Prune those two names from:

- `control_flow::lower::mod`
- `control_flow::lower::planner_compat`

Also avoid preserving a root plan re-export for `CoreBranchArmPlan` just for
the verifier tests; import the type from `plan::branchn` instead.

Keep the rest of the lower facade unchanged for this slice.

## Boundaries

- Do not move `PlanLowerer`.
- Do not touch `tags`, route labels, or planner build outcomes.
- Do not change verifier assertions or route behavior.
- Do not prune production lower exports in this card.
- Do not reopen broad `lower::planner_compat` ownership work without a new
  focused BoxShape lane.

## Acceptance

```bash
cargo fmt
cargo test verifier --lib
cargo check --release --bin hakorune -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

- Verifier tests now import `CoreBranchArmPlan` and `CoreIfJoin` directly from
  their plan owners.
- The lower facade no longer exports those two test-only names.
- The plan root no longer keeps a `CoreBranchArmPlan` re-export solely for
  verifier tests.
- Lowering behavior and planner compatibility wiring are unchanged.
