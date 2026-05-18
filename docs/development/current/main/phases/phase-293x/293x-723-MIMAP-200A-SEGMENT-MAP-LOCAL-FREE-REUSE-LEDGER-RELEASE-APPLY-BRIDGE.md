# 293x-723 MIMAP-200A Segment Map Local Free Reuse Ledger Release Apply Bridge

Status: landed
Date: 2026-05-18

## Decision

Connect the segment-map local-free reuse ledger release bridge to the existing
source-ledger release apply route.

## Context

MIMAP-198A closed the segment-map local-free reuse ledger release bridge pack.
MIMAP-200A keeps the same scalar/model boundary and applies that
segment-map-derived release row back to the source local-free reuse ledger:

```text
segment-map local-free reuse ledger release row
  -> source local-free reuse ledger release apply owner
```

## Scope

- Add proof app:
  `apps/hako-alloc-segment-map-local-free-reuse-ledger-release-apply-bridge-proof/`.
- Add SSOT:
  `docs/development/current/main/design/hako-alloc-segment-map-local-free-reuse-ledger-release-apply-bridge-ssot.md`.
- Add L2 guard:
  `tools/checks/k2_wide_hako_alloc_segment_map_local_free_reuse_ledger_release_apply_bridge_guard.sh`.
- Add proof manifest row with `validation_profile = "scalar-mir"` and
  `exe = "deferred-to-closeout"`.

## Stop Lines

- No real segment allocation/free execution.
- No raw pointer residence or pointer-derived lookup.
- No real segment-map mutation.
- No real free-list mutation.
- No direct page-array mutation outside explicit modeled page owners.
- No mutation of the release owner by the source ledger.
- No dependency on `segment_allocation_modeled_ledger_box.hako`,
  `recordModeledConsume`, or `releaseModeledToken` from this bridge.
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
bash tools/checks/k2_wide_hako_alloc_segment_map_local_free_reuse_ledger_release_apply_bridge_guard.sh --level L0
bash tools/checks/k2_wide_hako_alloc_segment_map_local_free_reuse_ledger_release_apply_bridge_guard.sh
bash tools/checks/run_proof_app.sh --only MIMAP-200A
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Next

```text
MIMAP-201A post-segment-map-local-free-reuse-ledger-release-apply-bridge row selection
```
