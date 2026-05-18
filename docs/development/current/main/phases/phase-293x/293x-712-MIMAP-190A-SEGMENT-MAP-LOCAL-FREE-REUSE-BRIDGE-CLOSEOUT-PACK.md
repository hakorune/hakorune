# 293x-712 MIMAP-190A Segment Map Local Free Reuse Bridge Closeout Pack

Status: landed
Date: 2026-05-18

## Decision

Close the segment-map local-free reuse bridge pack with representative
exact-MIR L3 EXE evidence.

## Context

MIMAP-188A proved the daily L2 behavior:

```text
segment-map released-span row
  -> modeled local-free integration owner
  -> modeled local-free reuse owner
     -> HakoAllocPageModel.acquire(size)
```

MIMAP-190A keeps that behavior in the
`segment-map-local-free-reuse-bridge` pack and runs representative L3 evidence
for the pack.

## Scope

- Add closeout SSOT:
  `docs/development/current/main/design/hako-alloc-segment-map-local-free-reuse-bridge-closeout-ssot.md`.
- Add manifest-backed closeout guard:
  `tools/checks/k2_wide_hako_alloc_segment_map_local_free_reuse_bridge_closeout_guard.sh`.
- Keep `HakoAllocSegmentAllocationModeledLocalFreeReuse` report construction
  local to the owner through `finishReport(result)` so the closeout does not
  depend on a wide report-helper argument ABI.
- Keep daily validation on L2 through
  `bash tools/checks/run_proof_app.sh --closeout-pack segment-map-local-free-reuse-bridge --level L2`.

## Stop Lines

- No real segment allocation/free execution.
- No real free-list mutation.
- No direct page-array mutation outside explicit modeled page owners.
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
bash tools/checks/k2_wide_hako_alloc_segment_map_local_free_reuse_bridge_closeout_guard.sh
bash tools/checks/run_proof_app.sh --closeout-pack segment-map-local-free-reuse-bridge --level L2 --dry-run
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Next

```text
MIMAP-191A post-segment-map-local-free-reuse-bridge-closeout row selection
```
