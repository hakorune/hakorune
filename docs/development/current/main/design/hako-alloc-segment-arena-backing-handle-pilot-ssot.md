# Hako Alloc Segment Arena Backing Handle Pilot SSOT

Status: active
Decision: accepted
Date: 2026-05-21

## Purpose

Add a narrow arena backing handle pilot after the no-escape pointer residence
pilot. The handle is a bounded scalar token associated with the proof-scope
pointer residence. It is not arena release/recycle execution.

## Owner

```text
lang/src/hako_alloc/memory/segment_arena_backing_handle_pilot_box.hako
```

## Row

MIMAP-345A owns the arena backing handle pilot.

## Reason Vocabulary

| Reason | Meaning |
| --- | --- |
| `0` | arena backing handle accepted |
| `1` | no-escape pointer residence report missing |
| `2` | no-escape pointer residence report rejected |
| `3` | private pointer token invalid |
| `4` | arena handle token invalid |
| `5` | a still-closed execution seam was requested |

## Stop Lines

- No real release/recycle execution.
- No pointer-derived lookup or dereference.
- No real arena backing release or recycle.
- No real segment-map mutation.
- No real segment allocation/free execution.
- No atomic bitmap execution.
- No OSVM/page-source execution.
- No TLS, worker-local, worker scheduling, or source-level concurrency.
- No provider activation, host allocator replacement, hooks, or
  `#[global_allocator]`.
- No cross-function `Result` direct ABI or runtime sum materialization.
- No backend `.inc` matcher by app, box, owner, or row name.
