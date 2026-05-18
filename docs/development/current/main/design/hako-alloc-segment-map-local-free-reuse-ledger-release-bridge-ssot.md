# Hako Alloc Segment Map Local Free Reuse Ledger Release Bridge SSOT

Status: active
Date: 2026-05-18
Decision: accepted

## Purpose

Connect the segment-map local-free reuse ledger bridge to the existing modeled
local-free reuse ledger release owner.

MIMAP-196A keeps the same scalar/model boundary and proves:

```text
segment-map local-free reuse ledger row
  -> modeled local-free reuse ledger release owner
```

## Owner Boundaries

Source bridge owner:

```text
lang/src/hako_alloc/memory/segment_allocation_modeled_local_free_reuse_ledger_box.hako
```

Release owner:

```text
lang/src/hako_alloc/memory/segment_allocation_modeled_local_free_reuse_ledger_release_box.hako
```

MIMAP-196A must reuse the MIMAP-134A release owner instead of widening the
source reuse ledger or the bump-shaped modeled allocation ledger.

## Validation

Daily validation is L2:

```bash
bash tools/checks/k2_wide_hako_alloc_segment_map_local_free_reuse_ledger_release_bridge_guard.sh
```

The row is a scalar-composition bridge. EXE evidence is deferred to a future
closeout pack.

## Stop Lines

- No real segment allocation/free execution.
- No raw pointer residence or pointer-derived lookup.
- No real segment-map mutation.
- No real free-list mutation.
- No direct page-array mutation outside explicit modeled page owners.
- No mutation of the source reuse ledger by the release owner.
- No dependency on `segment_allocation_modeled_ledger_box.hako`,
  `recordModeledConsume`, or `releaseModeledToken` from the release owner.
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
MIMAP-197A post-segment-map-local-free-reuse-ledger-release-bridge row selection
```
