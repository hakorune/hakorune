# Hako Alloc Segment Arena Backing Pointer-Derived Lookup Execution Pilot SSOT

Status: active
Decision: accepted
Date: 2026-05-21

## Purpose

Open pointer-derived lookup execution as a bounded scalar lookup fact after the
arena backing handle pilot. This row may derive a lookup result token from the
private pointer token and arena handle token, but it must not dereference memory
or execute arena release/recycle.

## Owner

```text
lang/src/hako_alloc/memory/segment_arena_backing_pointer_derived_lookup_execution_pilot_box.hako
```

## Reason Vocabulary

| Reason | Meaning |
| --- | --- |
| `0` | pointer-derived lookup fact accepted |
| `1` | arena backing handle report missing |
| `2` | arena backing handle report rejected |
| `3` | private pointer token invalid |
| `4` | arena handle token invalid |
| `5` | lookup result token invalid |
| `6` | a still-closed execution seam was requested |

## Stop Lines

- No dereference.
- No real release/recycle execution.
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
