---
Status: SSOT
Decision: accepted
Date: 2026-05-17
Scope: MIMAP-088A segment allocation readiness scalar contract.
Related:
  - docs/development/current/main/design/hako-alloc-segment-lifecycle-scalar-state-ssot.md
  - docs/development/current/main/design/hako-alloc-segment-page-membership-scalar-ssot.md
  - docs/development/current/main/phases/phase-293x/293x-585-MIMAP-088A-SEGMENT-ALLOCATION-READINESS-SCALAR-CONTRACT.md
  - lang/src/hako_alloc/memory/segment_allocation_readiness_scalar_box.hako
  - apps/hako-alloc-segment-allocation-readiness-scalar-proof/
---

# Hako Alloc Segment Allocation Readiness Scalar SSOT

## Decision

`MIMAP-088A` adds a proof-only scalar contract for segment allocation
readiness.

The row classifies `segment_id`, `page_id`, `segment_state`, `page_used`,
`page_capacity`, and request block count without opening segment
allocation/free execution, arena backing allocation, raw pointer residence,
segment-map lookup, atomic bitmap execution, OSVM execution, real scheduling,
provider activation, or backend matchers.

## Owner

```text
lang/src/hako_alloc/memory/segment_allocation_readiness_scalar_box.hako
```

Responsibilities:

```text
classify scalar allocation-readiness facts
validate tiny same-owner page/capacity/request counters
reject unsupported substrate requirements with stable reasons
report inactive execution / raw pointer / segment map / arena / atomic / OSVM / thread / provider flags
```

Non-responsibilities:

```text
segment allocation/free execution
arena backing allocation
raw pointer residence
segment-map pointer membership or lookup
atomic bitmap claim/unclaim
page-source or OSVM execution
real thread scheduling
provider activation / hooks / host allocator replacement
backend app/name matcher
```

## Reason Vocabulary

| Reason | Meaning |
| ---: | --- |
| `0` | accepted scalar allocation-readiness fact |
| `1` | invalid segment/page/counter/request shape |
| `2` | request does not fit available page capacity |
| `3` | raw pointer residence is required |
| `4` | segment-map pointer lookup is required |
| `5` | arena backing allocation is required |
| `6` | atomic bitmap execution is required |
| `7` | OSVM execution is required |
| `8` | unknown segment state |
| `9` | segment state does not support allocation readiness |
| `10` | real thread scheduling is required |
| `11` | provider activation is requested |
| `12` | segment allocation/free execution is requested |

## Unsupported Requirement Code

| Code | Requirement |
| ---: | --- |
| `0` | no unsupported requirement |
| `1` | raw pointer residence |
| `2` | segment-map pointer lookup |
| `3` | arena backing allocation |
| `4` | atomic bitmap execution |
| `5` | OSVM execution |
| `6` | real thread scheduling |
| `7` | provider activation |
| `8` | segment allocation/free execution |

## Readiness Policy

Scalar allocation readiness is accepted only for active segments:

```text
segment_state == Active
request_blocks > 0
page_used + request_blocks <= page_capacity
unsupported_requirement == 0
```

Other known segment states remain useful proof states, but this row does not
reactivate or mutate them before allocation.

## Stop Lines

No part of `MIMAP-088A` may add:

```text
segment allocation/free execution
arena backing allocation
raw pointer residence
segment-map pointer membership or lookup
atomic bitmap claim/unclaim
page-source call
OSVM execution, unreserve, or release
real thread scheduling
worker spawning
source-level concurrency semantics
provider activation
hooks
host allocator replacement
backend app/name matcher
```
