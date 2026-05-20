# Hako Alloc Segment Arena Backing No-Escape Pointer Residence Pilot SSOT

Status: active
Decision: accepted
Date: 2026-05-21

## Purpose

Open the first small real seam after the remaining prerequisite ledger closeout:
a no-escape pointer residence pilot represented by a private proof-scope token.

This row is deliberately narrower than pointer-derived lookup. It records that
an accepted remaining-prerequisite ledger can produce a no-escape residence
token, while keeping dereference, lookup, arena release/recycle, segment-map,
atomic bitmap, OSVM, worker/TLS, provider activation, and backend matchers
closed.

## Owner

```text
lang/src/hako_alloc/memory/segment_arena_backing_no_escape_pointer_residence_pilot_box.hako
```

## Row

MIMAP-344A owns the no-escape pointer residence pilot.

## Reason Vocabulary

| Reason | Meaning |
| --- | --- |
| `0` | no-escape pointer residence accepted |
| `1` | remaining prerequisite ledger report missing |
| `2` | remaining prerequisite ledger report rejected |
| `3` | private pointer token invalid |
| `4` | return escape would be required |
| `5` | storage escape would be required |
| `6` | alias escape would be required |
| `7` | a still-closed execution seam was requested |

## Validation

MIMAP-344A uses daily `scalar-mir` L2 validation. L3 evidence remains reserved
for a later closeout or a backend-facing route change.

## Stop Lines

- No real release/recycle execution.
- No pointer-derived lookup or dereference.
- No real arena backing allocation, release, or recycle.
- No real segment-map mutation.
- No real segment allocation/free execution.
- No atomic bitmap execution.
- No OSVM/page-source execution.
- No TLS, worker-local, worker scheduling, or source-level concurrency.
- No provider activation, host allocator replacement, hooks, or
  `#[global_allocator]`.
- No cross-function `Result` direct ABI or runtime sum materialization.
- No backend `.inc` matcher by app, box, owner, or row name.
