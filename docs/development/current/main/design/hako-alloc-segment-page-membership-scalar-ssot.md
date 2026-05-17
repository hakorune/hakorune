---
Status: SSOT
Decision: accepted
Date: 2026-05-17
Scope: MIMAP-085A segment page membership scalar contract.
Related:
  - docs/development/current/main/design/hako-alloc-segment-lifecycle-scalar-state-ssot.md
  - docs/development/current/main/phases/phase-293x/293x-572-MIMAP-085A-SEGMENT-PAGE-MEMBERSHIP-SCALAR-CONTRACT.md
  - lang/src/hako_alloc/memory/segment_page_membership_scalar_box.hako
  - apps/hako-alloc-segment-page-membership-scalar-proof/
---

# Hako Alloc Segment Page Membership Scalar SSOT

## Decision

`MIMAP-085A` adds a proof-only scalar membership contract between segment
lifecycle vocabulary and page-local vocabulary.

The row classifies `segment_id`, `page_id`, `slice_index`, `slice_count`,
`segment_state`, `page_used`, and `page_capacity` without opening raw pointer
residence, segment-map pointer lookup, arena backing allocation, atomic bitmap
execution, OSVM execution, real scheduling, provider activation, or backend
matchers.

## Owner

```text
lang/src/hako_alloc/memory/segment_page_membership_scalar_box.hako
```

Responsibilities:

```text
classify scalar segment/page membership facts
validate tiny same-owner page/slice counters
reject unsupported substrate requirements with stable reasons
report inactive raw pointer / segment map / arena / atomic / OSVM / thread / provider flags
```

Non-responsibilities:

```text
segment allocation/free execution
arena backing allocation
raw pointer residence
segment-map pointer membership
atomic bitmap claim/unclaim
page-source or OSVM execution
real thread scheduling
provider activation / hooks / host allocator replacement
backend app/name matcher
```

## Reason Vocabulary

| Reason | Meaning |
| ---: | --- |
| `0` | accepted scalar segment/page membership |
| `1` | invalid segment/page/slice/counter shape |
| `2` | raw pointer residence is required |
| `3` | segment-map pointer lookup is required |
| `4` | arena backing allocation is required |
| `5` | atomic bitmap execution is required |
| `6` | OSVM execution is required |
| `7` | unknown segment state |
| `8` | segment state does not support page membership |
| `9` | real thread scheduling is required |
| `10` | provider activation is requested |

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

## Membership State Policy

Scalar page membership is accepted only when `segment_state` is one of:

```text
Active
PurgeScheduled
Purged
Abandoned
Reclaimed
```

`Reserved` and `Freed` are known states but do not support page membership in
this proof-only row.

## Stop Lines

No part of `MIMAP-085A` may add:

```text
segment allocation/free execution
arena backing allocation
raw pointer residence
segment-map pointer membership
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

