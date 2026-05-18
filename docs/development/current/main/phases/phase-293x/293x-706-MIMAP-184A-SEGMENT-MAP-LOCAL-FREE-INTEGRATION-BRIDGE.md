# 293x-706 MIMAP-184A Segment Map Local Free Integration Bridge

Status: landed
Date: 2026-05-18

## Decision

Add a segment-map local-free integration bridge row.

## Context

MIMAP-182A closed the segment-map local-free page-apply bridge pack, and
MIMAP-183A selected the next small allocator row. The next step is to prove
that a segment-map-derived released-span row can enter the existing MIMAP-119A
modeled local-free integration owner.

The row proves:

```text
segment-map released-span row
  -> modeled local-free integration owner
     -> local-free candidate ledger
     -> modeled local-free apply-plan ledger
     -> modeled local-free page-apply row
```

## Scope

- Add proof app:
  `apps/hako-alloc-segment-map-local-free-integration-bridge-proof/`.
- Add L2 guard:
  `tools/checks/k2_wide_hako_alloc_segment_map_local_free_integration_bridge_guard.sh`.
- Add SSOT:
  `docs/development/current/main/design/hako-alloc-segment-map-local-free-integration-bridge-ssot.md`.
- Register the proof app in `tools/checks/proof_apps.toml`.

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
bash tools/checks/k2_wide_hako_alloc_segment_map_local_free_integration_bridge_guard.sh
bash tools/checks/run_proof_app.sh --only MIMAP-184A
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Next

```text
MIMAP-185A post-segment-map-local-free-integration-bridge row selection
```
