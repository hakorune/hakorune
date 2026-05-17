---
Status: SSOT
Decision: accepted
Date: 2026-05-17
Scope: MIMAP-082A segment lifecycle scalar state contract.
Related:
  - docs/development/current/main/design/mimalloc-lifecycle-rewrite-blueprint-ssot.md
  - docs/development/current/main/design/hako-alloc-segment-arena-bitmap-inventory-ssot.md
  - docs/development/current/main/phases/phase-293x/293x-569-MIMAP-082A-SEGMENT-LIFECYCLE-SCALAR-STATE-CONTRACT.md
  - lang/src/hako_alloc/memory/segment_lifecycle_scalar_state_box.hako
  - apps/hako-alloc-segment-lifecycle-scalar-state-proof/
---

# Hako Alloc Segment Lifecycle Scalar State SSOT

## Decision

`MIMAP-082A` adds a proof-only scalar segment lifecycle contract.

The row names segment states and legal transitions from the lifecycle rewrite
blueprint while keeping pointer residence, atomic bitmap execution, OSVM,
thread scheduling, provider activation, and backend matchers inactive.

## Owner

```text
lang/src/hako_alloc/memory/segment_lifecycle_scalar_state_box.hako
```

Responsibilities:

```text
classify scalar segment lifecycle state transitions
validate tiny same-owner segment counters
reject unsupported substrate requirements with stable reasons
report inactive raw pointer / atomic bitmap / OSVM / thread / provider flags
```

Non-responsibilities:

```text
segment allocation/free execution
arena backing allocation
raw pointer residence
atomic bitmap claim/unclaim
page-source or OSVM execution
real thread scheduling
provider activation / hooks / host allocator replacement
backend app/name matcher
```

## State Vocabulary

| Code | State |
| ---: | --- |
| `0` | `Reserved` |
| `1` | `Active` |
| `2` | `PurgeScheduled` |
| `3` | `Purged` |
| `4` | `Abandoned` |
| `5` | `Reclaimed` |
| `6` | `Freed` |

## Transition Vocabulary

| Code | Transition |
| ---: | --- |
| `10` | `Reserved -> Active` |
| `11` | `Active -> PurgeScheduled` |
| `12` | `PurgeScheduled -> Purged` |
| `13` | `Active -> Abandoned` |
| `14` | `Abandoned -> Reclaimed` |
| `15` | `Reclaimed -> Active` |
| `16` | `Active -> Freed` |
| `17` | `Purged -> Freed` |

## Reason Vocabulary

| Reason | Meaning |
| ---: | --- |
| `0` | accepted scalar lifecycle transition |
| `1` | invalid segment/count shape |
| `2` | raw pointer residence is required |
| `3` | atomic bitmap execution is required |
| `4` | OSVM execution is required |
| `5` | real thread scheduling is required |
| `6` | provider activation is requested |
| `7` | unknown segment state |
| `8` | invalid segment lifecycle transition |

## Unsupported Requirement Code

| Code | Requirement |
| ---: | --- |
| `0` | no unsupported requirement |
| `1` | raw pointer residence |
| `2` | atomic bitmap execution |
| `3` | OSVM execution |
| `4` | real thread scheduling |
| `5` | provider activation |

## Inactive Flags

Accepted reports must keep:

```text
would_use_raw_pointer = 0
would_execute_atomic_bitmap = 0
would_call_osvm = 0
would_run_thread = 0
would_activate_provider = 0
would_replace_process_allocator = 0
would_add_backend_matcher = 0
```

## Stop Lines

No part of `MIMAP-082A` may add:

```text
segment allocation/free execution
arena backing allocation
raw pointer residence
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

