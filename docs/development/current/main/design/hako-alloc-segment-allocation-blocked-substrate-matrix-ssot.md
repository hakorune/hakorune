---
Status: SSOT
Decision: accepted
Date: 2026-05-18
Scope: MIMAP-149A segment allocation blocked-substrate matrix proof.
Related:
  - docs/development/current/main/design/hako-alloc-segment-allocation-readiness-scalar-ssot.md
  - docs/development/current/main/design/hako-alloc-segment-page-membership-scalar-ssot.md
  - docs/development/current/main/design/hako-alloc-segment-arena-bitmap-inventory-ssot.md
  - docs/development/current/main/phases/phase-293x/293x-669-MIMAP-149A-SEGMENT-ALLOCATION-BLOCKED-SUBSTRATE-MATRIX-PROOF.md
  - lang/src/hako_alloc/memory/segment_allocation_blocked_substrate_matrix_box.hako
  - apps/hako-alloc-segment-allocation-blocked-substrate-matrix-proof/
---

# Hako Alloc Segment Allocation Blocked Substrate Matrix SSOT

## Decision

`MIMAP-149A` adds a proof-only matrix for the hard substrate blockers that
remain closed between the current scalar segment allocation model and real
segment allocation/free.

The row composes already-landed scalar owners:

```text
HakoAllocSegmentAllocationReadinessScalar
HakoAllocSegmentPageMembershipScalar
HakoAllocSegmentArenaBitmapInventory
```

It does not open any blocked substrate. It only reports the current blocker
matrix so the next allocator row can choose one substrate boundary deliberately.

## Owner

```text
lang/src/hako_alloc/memory/segment_allocation_blocked_substrate_matrix_box.hako
```

Responsibilities:

```text
compose existing scalar segment readiness / membership / arena facts
report one stable row for each still-closed substrate blocker
publish inactive flags for execution/provider/backend surfaces
```

Non-responsibilities:

```text
real segment allocation/free execution
raw pointer residence
segment-map pointer lookup or membership execution
arena backing allocation
atomic bitmap claim/unclaim
page-source or OSVM execution
worker scheduling or source-level concurrency
provider activation / hooks / host allocator replacement
backend app/name matcher
```

## Matrix Rows

| Row | Blocker | Evidence owner |
| ---: | --- | --- |
| `1` | raw pointer residence | segment/arena/bitmap inventory |
| `2` | segment-map lookup | segment/page membership |
| `3` | arena backing allocation | segment/page membership |
| `4` | atomic bitmap execution | segment/arena/bitmap inventory |
| `5` | OSVM execution | segment/arena/bitmap inventory |
| `6` | real thread scheduling | allocation readiness |
| `7` | provider activation | segment/arena/bitmap inventory |
| `8` | real segment allocation/free execution | allocation readiness |

## Proof Surface

```text
apps/hako-alloc-segment-allocation-blocked-substrate-matrix-proof/
tools/checks/k2_wide_hako_alloc_segment_allocation_blocked_substrate_matrix_guard.sh
```

Expected proof output:

```text
matrix=0,8,3,255
accepted_reasons=0,0,0
blocker_reasons=2,3,4,3,4,10,5,12
blockers=1,1,1,1,1,1,1,1
inactive=0,0,0,0,0,0,0,0,0,0
summary=ok
```
