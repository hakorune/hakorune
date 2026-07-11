---
Status: Landed
Date: 2026-04-28
Scope: prune dead loop_scan_phi_vars_v0 recipe wrappers and unused evidence fields
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/mir/builder/control_flow/recipes/loop_scan_phi_vars_v0.rs
  - src/mir/builder/control_flow/facts/loop_scan_phi_vars_v0_shape_routes.rs
  - src/mir/builder/control_flow/plan/loop_scan_phi_vars_v0/pipeline.rs
---

# 291x-590: Loop Scan Phi Dead Evidence Prune

## Goal

Remove dead `loop_scan_phi_vars_v0` recipe baggage that no live code reads,
while keeping the route's active segment contract intact.

This is BoxShape-only cleanup. It does not change facts extraction, route
selection, or lowering behavior.

## Evidence

The remaining live readers only use:

- `recipe.inner_loop_search`
- `recipe.found_if_stmt`

The following had no live readers after 291x-589 and only remained as
dead-code evidence fields:

- `LoopScanPhiVarsV0Segments`
- `local_var_name_stmt`
- `local_j_stmt`
- `local_m_stmt`
- `local_found_stmt`
- `step_inc_stmt`

## Boundaries

- Keep `LoopScanPhiSegment`, `inner_loop_search`, and `found_if_stmt`.
- Remove only fields/types with zero live readers.
- Do not change the phi-vars lowering pipeline or matcher vocabulary.

## Acceptance

- No dead-code wrapper/type remains for `LoopScanPhiVarsV0Segments`.
- The removed recipe fields have no live readers under
  `src/mir/builder/control_flow`.
- `bash tools/checks/current_state_pointer_guard.sh` passes.
- `cargo check --release --bin hakorune` passes.
- `cargo fmt -- --check` passes.
- `git diff --check` passes.

## Result

- Deleted the unused `LoopScanPhiVarsV0Segments` wrapper.
- Removed five unused evidence fields from `LoopScanPhiVarsV0Recipe`.
- Simplified the shape-route constructors to only build the fields that active
  lowering still consumes.

## Verification

```bash
rg -n "LoopScanPhiVarsV0Segments|local_var_name_stmt|local_j_stmt|local_m_stmt|local_found_stmt|step_inc_stmt" src/mir/builder/control_flow -g'*.rs'
bash tools/checks/current_state_pointer_guard.sh
cargo fmt -- --check
cargo check --release --bin hakorune
git diff --check
```
