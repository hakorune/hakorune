# 293x-963 MIMAP-348A Atomic Bitmap Pilot

Status: selected current
Date: 2026-05-21

## Decision

Open atomic bitmap execution as the next narrow seam after segment-map mutation.

## Context

MIMAP-347A proved a bounded segment-map mutation fact from the accepted
pointer-derived lookup execution report. MIMAP-348A may use that fact to model
the first atomic bitmap pilot, but it must not dereference memory or execute
arena release/recycle.

## Scope

- Add an atomic bitmap pilot owner/proof/guard.
- Consume the MIMAP-347A segment-map mutation report.
- Publish bounded scalar bitmap facts for the atomic bitmap seam.
- Keep dereference, real arena backing release/recycle, OSVM/page-source,
  worker/TLS, provider activation, and backend matcher execution closed.

## Stop Lines

- No dereference.
- No real release/recycle execution.
- No real arena backing release or recycle.
- No OSVM/page-source execution.
- No TLS, worker-local, worker scheduling, or source-level concurrency.
- No provider activation, host allocator replacement, hooks, or
  `#[global_allocator]`.
- No cross-function `Result` direct ABI or runtime sum materialization.
- No backend `.inc` matcher by app, box, owner, or row name.

## Required Evidence

```text
bash tools/checks/k2_wide_hako_alloc_atomic_bitmap_pilot_guard.sh --level L2
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
