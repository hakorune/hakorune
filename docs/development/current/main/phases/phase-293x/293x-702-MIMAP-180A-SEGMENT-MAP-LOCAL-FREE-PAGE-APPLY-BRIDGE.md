# 293x-702 MIMAP-180A Segment Map Local Free Page Apply Bridge

Status: landed
Date: 2026-05-18

## Decision

Add a segment-map local-free page-apply bridge row.

## Context

MIMAP-178A closed the segment-map local-free apply-plan bridge pack. The next
small allocator row is to prove the same segment-map-derived apply-plan report
can feed the existing MIMAP-115A modeled page-apply owner.

The row proves:

```text
segment-map released-span row
  -> local-free candidate ledger row
  -> modeled local-free apply-plan ledger row
  -> modeled local-free page-apply row
```

## Scope

- Add proof app:
  `apps/hako-alloc-segment-map-local-free-page-apply-bridge-proof/`.
- Add L2 guard:
  `tools/checks/k2_wide_hako_alloc_segment_map_local_free_page_apply_bridge_guard.sh`.
- Add SSOT:
  `docs/development/current/main/design/hako-alloc-segment-map-local-free-page-apply-bridge-ssot.md`.
- Register the proof app in `tools/checks/proof_apps.toml`.

## Stop Lines

- No real segment allocation/free execution.
- No real free-list mutation.
- No direct page-array mutation outside `HakoAllocPageModel.releaseLocal`.
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
bash tools/checks/k2_wide_hako_alloc_segment_map_local_free_page_apply_bridge_guard.sh
bash tools/checks/run_proof_app.sh --only MIMAP-180A
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Next

```text
MIMAP-181A post-segment-map-local-free-page-apply-bridge row selection
```
