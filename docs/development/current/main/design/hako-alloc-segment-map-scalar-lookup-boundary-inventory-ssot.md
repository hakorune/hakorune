---
Status: SSOT
Decision: accepted
Date: 2026-05-18
Scope: MIMAP-151A segment-map scalar lookup boundary inventory.
Related:
  - docs/development/current/main/design/hako-alloc-segment-allocation-blocked-substrate-matrix-ssot.md
  - docs/development/current/main/design/mimalloc-substrate-representation-gap-ledger-ssot.md
  - docs/development/current/main/phases/phase-293x/293x-671-MIMAP-151A-SEGMENT-MAP-SCALAR-LOOKUP-BOUNDARY-INVENTORY.md
  - lang/src/hako_alloc/memory/segment_map_scalar_lookup_boundary_inventory_box.hako
  - apps/hako-alloc-segment-map-scalar-lookup-boundary-inventory-proof/
---

# Hako Alloc Segment Map Scalar Lookup Boundary Inventory SSOT

## Decision

`MIMAP-151A` adds an explicit-ID scalar lookup inventory for segment/page/slice
membership. It advances the segment-map boundary without deriving identity from
raw pointers.

Raw pointer residence remains parked behind a future `uses rawbuf` /
no-escape view capability. This row does not create a real segment map.

## Owner

```text
lang/src/hako_alloc/memory/segment_map_scalar_lookup_boundary_inventory_box.hako
```

Responsibilities:

```text
accept one known scalar segment/page/slice/generation row
reject unknown segment / wrong page / stale generation / out-of-range slice
reject raw-pointer lookup requests before any pointer-derived map opens
publish inactive execution/provider/backend flags
```

Non-responsibilities:

```text
raw pointer residence
real segment-map lookup execution
real segment allocation/free
arena backing allocation
atomic bitmap claim/unclaim
page-source or OSVM execution
worker scheduling or source-level concurrency
provider activation / hooks / host allocator replacement
backend app/name matcher
```

## Reason Codes

| Code | Meaning |
| ---: | --- |
| `0` | accepted |
| `1` | invalid scalar shape |
| `2` | raw-pointer lookup requested |
| `3` | unknown segment |
| `4` | wrong page for known segment |
| `5` | stale generation |
| `6` | out-of-range slice |

## Proof Surface

```text
apps/hako-alloc-segment-map-scalar-lookup-boundary-inventory-proof/
tools/checks/k2_wide_hako_alloc_segment_map_scalar_lookup_boundary_inventory_guard.sh
```

Expected proof output:

```text
lookup=1,0,0,70,7,3,16,1
rejects=3,4,5,6,2
inactive=0,0,0,0,0,0,0,0,0
counts=6,1,5,1,1,1,1,1,2
summary=ok
```
