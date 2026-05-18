# 293x-700 MIMAP-178A Segment Map Local Free Apply Plan Bridge Closeout Pack

Status: landed
Date: 2026-05-18

## Decision

Close the segment-map local-free apply-plan bridge pack with representative
exact-MIR L3 EXE evidence.

## Context

MIMAP-176A proved the daily L2 behavior:

```text
segment-map released-span row
  -> local-free candidate ledger row
  -> modeled local-free apply-plan ledger row
```

MIMAP-178A keeps that behavior in the
`segment-map-local-free-apply-plan-bridge` pack and runs the representative L3
evidence for the pack.

## Scope

- Add closeout SSOT:
  `docs/development/current/main/design/hako-alloc-segment-map-local-free-apply-plan-bridge-closeout-ssot.md`.
- Add manifest-backed closeout guard:
  `tools/checks/k2_wide_hako_alloc_segment_map_local_free_apply_plan_bridge_closeout_guard.sh`.
- Keep daily validation on L2 through
  `bash tools/checks/run_proof_app.sh --closeout-pack segment-map-local-free-apply-plan-bridge --level L2`.

## Stop Lines

- No real segment allocation/free execution.
- No real free-list mutation.
- No page-state mutation.
- No raw pointer residence or pointer-derived lookup.
- No real segment-map mutation.
- No arena backing allocation.
- No atomic bitmap execution.
- No OSVM/page-source execution.
- No TLS, worker-local, worker scheduling, or source-level concurrency.
- No provider activation, host allocator replacement, hooks, or
  `#[global_allocator]`.
- No cross-function `Result` direct ABI or runtime sum materialization.
- No backend `.inc` matcher by app or owner name.

## Required Evidence

```text
bash tools/checks/k2_wide_hako_alloc_segment_map_local_free_apply_plan_bridge_closeout_guard.sh
bash tools/checks/run_proof_app.sh --closeout-pack segment-map-local-free-apply-plan-bridge --level L2 --dry-run
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Next

```text
MIMAP-179A post-segment-map-local-free-apply-plan-bridge-closeout row selection
```
