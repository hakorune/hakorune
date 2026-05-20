# Hako Alloc Atomic Bitmap Pilot SSOT

Status: active
Decision: accepted
Date: 2026-05-21

## Purpose

Open the first bounded atomic bitmap fact after segment-map mutation. This row
may record a scalar bitmap token from an accepted segment-map mutation report,
but it must not use real atomic primitives, dereference memory, or execute
arena release/recycle.

## Owner

```text
lang/src/hako_alloc/memory/atomic_bitmap_pilot_box.hako
```

## Reason Vocabulary

| Reason | Meaning |
| --- | --- |
| `0` | atomic bitmap fact accepted |
| `1` | segment-map mutation report missing |
| `2` | segment-map mutation report rejected |
| `3` | dereferenceable pointer request rejected |
| `4` | segment-map mutation token invalid |
| `5` | atomic bitmap token invalid |
| `6` | a still-closed execution seam was requested |

## Stop Lines

- No real atomic primitive, CAS, fetch-add, or backend atomic lowering.
- No dereference.
- No real release/recycle execution.
- No real arena backing release or recycle.
- No OSVM/page-source execution.
- No TLS, worker-local, worker scheduling, or source-level concurrency.
- No provider activation, host allocator replacement, hooks, or
  `#[global_allocator]`.
- No cross-function `Result` direct ABI or runtime sum materialization.
- No backend `.inc` matcher by app, box, owner, or row name.
