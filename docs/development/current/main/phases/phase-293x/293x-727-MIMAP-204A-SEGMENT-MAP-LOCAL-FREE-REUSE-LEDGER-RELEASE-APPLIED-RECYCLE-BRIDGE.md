# 293x-727 MIMAP-204A Segment Map Local Free Reuse Ledger Release-Applied Recycle Bridge

Status: landed
Date: 2026-05-18

## Decision

Connect the segment-map local-free reuse ledger release apply bridge to the
existing source-ledger release-applied recycle route.

## Context

MIMAP-202A closed the release apply bridge pack with representative exact-MIR
L3 EXE evidence. MIMAP-204A keeps the same scalar/model boundary and proves the
next smallest behavior:

```text
segment-map local-free reuse ledger release row
  -> source local-free reuse ledger release apply owner
  -> same modeled reuse token can be recorded again as a new live row
```

## Scope

- Add proof app:
  `apps/hako-alloc-segment-map-local-free-reuse-ledger-release-applied-recycle-bridge-proof/`.
- Add SSOT:
  `docs/development/current/main/design/hako-alloc-segment-map-local-free-reuse-ledger-release-applied-recycle-bridge-ssot.md`.
- Add L2 guard:
  `tools/checks/k2_wide_hako_alloc_segment_map_local_free_reuse_ledger_release_applied_recycle_bridge_guard.sh`.
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
- No new segment-map-specific recycle owner.
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
bash tools/checks/k2_wide_hako_alloc_segment_map_local_free_reuse_ledger_release_applied_recycle_bridge_guard.sh --level L0
bash tools/checks/k2_wide_hako_alloc_segment_map_local_free_reuse_ledger_release_applied_recycle_bridge_guard.sh
bash tools/checks/run_proof_app.sh --only MIMAP-204A
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Next

```text
MIMAP-205A post-segment-map-local-free-reuse-ledger-release-applied-recycle-bridge row selection
```
