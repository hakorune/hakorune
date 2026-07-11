---
Status: Landed
Date: 2026-04-28
Scope: prune plan/features exit_branch facade
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/mir/builder/control_flow/plan/features/mod.rs
  - src/mir/builder/control_flow/plan/parts/exit.rs
  - src/mir/builder/control_flow/plan/parts/exit_branch.rs
---

# 291x-633: Exit Branch Feature Facade Prune

## Goal

Remove the remaining `plan::features::exit_branch` facade after its real
responsibilities had moved to the `parts` SSOT layer.

This is BoxShape cleanup only. It does not change exit semantics, accepted loop
shapes, route ordering, or lowering behavior.

## Evidence

The active prelude/exit-branch lowering implementation already lived in
`plan::parts::exit_branch`. The remaining `features::exit_branch` module mostly
delegated to `parts` and owned only small `CoreExitPlan` constructor helpers.

The live callsites only needed:

- return/break/continue `CoreExitPlan` constructors
- return/break/continue one-node `CorePlan::Exit` helpers

Those helpers belong with `plan::parts::exit`, which already owns exit lowering
and PHI-arg exit construction.

## Boundaries

- Move only trivial exit constructors to `parts::exit`.
- Repoint existing callsites to `parts::exit`.
- Delete only the now-unused `features::exit_branch` facade.
- Do not modify route predicates, recipe contracts, or branch lowering logic.

## Acceptance

- `cargo fmt -- --check` passes.
- `cargo check --release --bin hakorune -q` passes.
- `bash tools/checks/current_state_pointer_guard.sh` passes.
- `git diff --check` passes.

## Result

- Added return/break/continue exit constructors to `parts::exit`.
- Repointed generic-loop and BranchN callsites to `parts::exit`.
- Removed `features::exit_branch` from the feature module surface.
- Reworded the `parts::exit_branch` module comment as the current SSOT owner.

## Verification

```bash
cargo fmt -- --check
cargo check --release --bin hakorune -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
