# Hako Alloc Segment Map Local Free Reuse Ledger Release-Applied Recycle Bridge SSOT

Status: active
Date: 2026-05-18
Decision: accepted

## Purpose

Connect the segment-map local-free reuse ledger release apply bridge to the
existing source-ledger release-applied recycle route.

MIMAP-204A keeps the same scalar/model boundary and proves:

```text
segment-map local-free reuse ledger release row
  -> source local-free reuse ledger release apply owner
  -> source row becomes non-live
  -> same modeled reuse token can be recorded again as a new live row
```

## Owner Boundaries

Source ledger owner:

```text
lang/src/hako_alloc/memory/segment_allocation_modeled_local_free_reuse_ledger_box.hako
```

Release owner:

```text
lang/src/hako_alloc/memory/segment_allocation_modeled_local_free_reuse_ledger_release_box.hako
```

MIMAP-204A must reuse the existing `applyReuseLedgerRelease` and
`recordLocalFreeReuse` routes instead of adding a segment-map-specific recycle
owner, mutating page state, or widening the bump-shaped modeled ledger.

## Validation

Daily validation is L2:

```bash
bash tools/checks/k2_wide_hako_alloc_segment_map_local_free_reuse_ledger_release_applied_recycle_bridge_guard.sh
```

The row is a scalar-composition bridge. EXE evidence is deferred to a future
closeout pack.

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

## Next

```text
MIMAP-205A post-segment-map-local-free-reuse-ledger-release-applied-recycle-bridge row selection
```
