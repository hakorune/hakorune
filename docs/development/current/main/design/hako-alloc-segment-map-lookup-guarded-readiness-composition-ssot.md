---
Status: SSOT
Decision: accepted
Date: 2026-05-18
Scope: MIMAP-153A segment-map lookup guarded readiness composition.
Related:
  - docs/development/current/main/design/hako-alloc-segment-map-scalar-lookup-boundary-inventory-ssot.md
  - docs/development/current/main/design/hako-alloc-segment-page-membership-scalar-ssot.md
  - docs/development/current/main/design/hako-alloc-segment-allocation-readiness-scalar-ssot.md
  - docs/development/current/main/phases/phase-293x/293x-673-MIMAP-153A-SEGMENT-MAP-LOOKUP-GUARDED-READINESS-COMPOSITION.md
  - lang/src/hako_alloc/memory/segment_map_lookup_guarded_readiness_composition_box.hako
  - apps/hako-alloc-segment-map-lookup-guarded-readiness-composition-proof/
---

# Hako Alloc Segment Map Lookup Guarded Readiness Composition SSOT

## Decision

`MIMAP-153A` composes explicit-ID segment-map scalar lookup with the existing
segment/page membership and allocation-readiness scalar facts. The lookup result
is the gate: membership and readiness are evaluated only after lookup accepts.

The row remains proof-only. It does not create a real segment map and does not
derive identity from raw pointers.

## Owner

```text
lang/src/hako_alloc/memory/segment_map_lookup_guarded_readiness_composition_box.hako
```

Responsibilities:

```text
call explicit-ID scalar lookup first
compose accepted lookup with segment/page membership facts
compose accepted membership with allocation-readiness facts
publish stable composition reject reasons
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
| `0` | lookup, membership, and readiness accepted |
| `1` | lookup rejected |
| `2` | membership rejected after lookup accepted |
| `3` | readiness rejected after lookup and membership accepted |

Subreason fields carry the original owner reason:

```text
lookup_reason
membership_reason
readiness_reason
```

## Proof Surface

```text
apps/hako-alloc-segment-map-lookup-guarded-readiness-composition-proof/
tools/checks/k2_wide_hako_alloc_segment_map_lookup_guarded_readiness_composition_guard.sh
```

Expected proof output:

```text
composition=1,0,0,0,0,70,7,3,16,1,2,8,3,6
reject_reasons=1,2,3,1
subreasons=3,8,2,2
inactive=0,0,0,0,0,0,0,0,0
counts=5,1,4,2,1,1,1,1
summary=ok
```
