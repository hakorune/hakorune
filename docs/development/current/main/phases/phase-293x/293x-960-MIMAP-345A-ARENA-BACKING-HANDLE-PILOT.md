# 293x-960 MIMAP-345A Arena Backing Handle Pilot

Status: landed
Date: 2026-05-21

## Decision

Add the next narrow seam after no-escape pointer residence: an arena backing
handle pilot.

## Context

MIMAP-344A proved a private proof-scope no-escape pointer residence token. The
next step may introduce an arena backing handle/token that can be associated
with that residence, but it must not execute arena release/recycle or pointer
derived lookup.

## Scope

- Add an arena backing handle owner/proof/guard.
- Consume the MIMAP-344A no-escape pointer residence report.
- Publish a bounded handle token and scalar report facts.
- Keep release/recycle, pointer-derived lookup, dereference, segment-map,
  atomic bitmap, OSVM, worker/TLS, provider activation, and backend matcher
  execution closed.

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

## Required Evidence

```text
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_handle_pilot_guard.sh --level L2
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

Landed the arena backing handle pilot with L2 VM/MIR evidence. MIMAP-346A is
selected to open pointer-derived lookup execution as the next narrow seam while
keeping dereference, arena release/recycle, segment-map mutation, atomic bitmap,
OSVM, worker/TLS, provider activation, and backend matcher execution closed.
