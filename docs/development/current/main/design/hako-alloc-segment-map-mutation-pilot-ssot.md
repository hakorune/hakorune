# Hako Alloc Segment-Map Mutation Pilot SSOT

Status: active
Decision: accepted
Date: 2026-05-21

## Purpose

Open the first bounded segment-map mutation fact after pointer-derived lookup
execution. This row may record a scalar mutation token from an accepted
pointer-derived lookup report, but it must not dereference memory or execute
arena release/recycle.

## Owner

```text
lang/src/hako_alloc/memory/segment_map_mutation_pilot_box.hako
```

## Reason Vocabulary

| Reason | Meaning |
| --- | --- |
| `0` | segment-map mutation fact accepted |
| `1` | pointer-derived lookup report missing |
| `2` | pointer-derived lookup report rejected |
| `3` | dereferenceable pointer request rejected |
| `4` | lookup result token invalid |
| `5` | segment-map mutation token invalid |
| `6` | a still-closed execution seam was requested |

## Stop Lines

- No dereference.
- No real release/recycle execution.
- No real arena backing release or recycle.
- No atomic bitmap execution.
- No OSVM/page-source execution.
- No TLS, worker-local, worker scheduling, or source-level concurrency.
- No provider activation, host allocator replacement, hooks, or
  `#[global_allocator]`.
- No cross-function `Result` direct ABI or runtime sum materialization.
- No backend `.inc` matcher by app, box, owner, or row name.
