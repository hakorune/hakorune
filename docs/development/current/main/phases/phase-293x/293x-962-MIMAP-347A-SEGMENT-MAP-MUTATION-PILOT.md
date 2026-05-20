# 293x-962 MIMAP-347A Segment-Map Mutation Pilot

Status: landed
Date: 2026-05-21

## Decision

Open segment-map mutation as the next narrow seam after pointer-derived lookup
execution.

## Context

MIMAP-344A opened private no-escape pointer residence, MIMAP-345A associated
that residence with a bounded arena backing handle, and MIMAP-346A derived a
non-dereferenceable pointer-derived lookup fact. MIMAP-347A may use those facts
to model the first segment-map mutation pilot, but it must not dereference
memory or execute arena release/recycle.

## Scope

- Add a segment-map mutation pilot owner/proof/guard.
- Consume the MIMAP-346A pointer-derived lookup execution report.
- Publish bounded scalar mutation facts for the segment-map seam.
- Keep dereference, real arena backing release/recycle, atomic bitmap,
  OSVM/page-source, worker/TLS, provider activation, and backend matcher
  execution closed.

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

## Required Evidence

```text
bash tools/checks/k2_wide_hako_alloc_segment_map_mutation_pilot_guard.sh --level L2
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

Landed the segment-map mutation pilot with L2 VM/MIR evidence. MIMAP-348A is
selected to open an atomic bitmap pilot as the next narrow seam while keeping
dereference, real arena backing release/recycle, OSVM/page-source, worker/TLS,
provider activation, and backend matcher execution closed.
