# 293x-714 MIMAP-192A Segment Map Local Free Reuse Ledger Bridge

Status: landed
Date: 2026-05-18

## Decision

Connect the segment-map local-free reuse bridge to the existing modeled
local-free reuse ledger owner.

## Context

MIMAP-190A closed the segment-map local-free reuse bridge pack. MIMAP-192A
keeps the same scalar/model boundary and records the successful reuse report in
the existing MIMAP-130A reuse ledger:

```text
segment-map released-span row
  -> modeled local-free reuse owner
  -> modeled local-free reuse ledger owner
```

## Scope

- Add proof app:
  `apps/hako-alloc-segment-map-local-free-reuse-ledger-bridge-proof/`.
- Add SSOT:
  `docs/development/current/main/design/hako-alloc-segment-map-local-free-reuse-ledger-bridge-ssot.md`.
- Add L2 guard:
  `tools/checks/k2_wide_hako_alloc_segment_map_local_free_reuse_ledger_bridge_guard.sh`.
- Add proof manifest row with `validation_profile = "scalar-mir"` and
  `exe = "deferred-to-closeout"`.

## Stop Lines

- No real segment allocation/free execution.
- No raw pointer residence or pointer-derived lookup.
- No real segment-map mutation.
- No real free-list mutation.
- No direct page-array mutation outside explicit modeled page owners.
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
bash tools/checks/k2_wide_hako_alloc_segment_map_local_free_reuse_ledger_bridge_guard.sh --level L0
bash tools/checks/k2_wide_hako_alloc_segment_map_local_free_reuse_ledger_bridge_guard.sh
bash tools/checks/run_proof_app.sh --only MIMAP-192A
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Next

```text
MIMAP-193A post-segment-map-local-free-reuse-ledger-bridge row selection
```
