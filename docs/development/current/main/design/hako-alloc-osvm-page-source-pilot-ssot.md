# Hako Alloc OSVM Page-Source Pilot SSOT

Status: active
Decision: accepted
Date: 2026-05-21

## Purpose

Open a bounded OSVM/page-source fact after the atomic bitmap pilot. This row may
record a scalar page-source token from an accepted atomic bitmap report, but it
must not activate providers, replace the host allocator, or expose hooks.

## Owner

```text
lang/src/hako_alloc/memory/osvm_page_source_pilot_box.hako
```

## Reason Vocabulary

| Reason | Meaning |
| --- | --- |
| `0` | OSVM/page-source fact accepted |
| `1` | atomic bitmap report missing |
| `2` | atomic bitmap report rejected |
| `3` | dereferenceable pointer request rejected |
| `4` | atomic bitmap token invalid |
| `5` | OSVM/page-source token invalid |
| `6` | a still-closed execution seam was requested |

## Stop Lines

- No dereference.
- No real release/recycle execution.
- No real arena backing release or recycle.
- No TLS, worker-local, worker scheduling, or source-level concurrency.
- No provider activation, host allocator replacement, hooks, or
  `#[global_allocator]`.
- No cross-function `Result` direct ABI or runtime sum materialization.
- No backend `.inc` matcher by app, box, owner, or row name.
