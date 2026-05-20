# 293x-961 MIMAP-346A Pointer-Derived Lookup Execution Pilot

Status: landed
Date: 2026-05-21

## Decision

Open pointer-derived lookup execution as the next narrow seam after the arena
backing handle pilot.

## Context

MIMAP-344A opened a no-escape pointer residence token, and MIMAP-345A associated
that token with an arena backing handle. MIMAP-346A may derive a bounded lookup
fact from those two tokens, but it must not dereference memory or execute
arena release/recycle.

## Scope

- Add a pointer-derived lookup execution owner/proof/guard.
- Consume the MIMAP-345A arena backing handle report.
- Publish a bounded lookup result token/fact.
- Keep dereference, arena release/recycle, segment-map mutation, atomic bitmap,
  OSVM, worker/TLS, provider activation, and backend matcher execution closed.

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

## Required Evidence

```text
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_pointer_derived_lookup_execution_pilot_guard.sh --level L2
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

Landed the pointer-derived lookup execution pilot with L2 VM/MIR evidence.
MIMAP-347A is selected to open a segment-map mutation pilot as the next narrow
seam while keeping dereference, real arena backing release/recycle, atomic
bitmap, OSVM/page-source, worker/TLS, provider activation, and backend matcher
execution closed.
