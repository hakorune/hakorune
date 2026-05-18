# 293x-780 MIMAP-257A Segment Arena Backing Modeled Arena Slot Diagnostics

Status: landed
Date: 2026-05-19

## Decision

Add observer-only diagnostics for the MIMAP-256A modeled arena-slot inventory.

## Context

MIMAP-256A records scalar/model arena-slot facts from an accepted modeled
residence arena-binding report. The next row should summarize slot counters
and reason categories before closeout.

## Scope

- Observe MIMAP-256A modeled arena-slot inventory counters.
- Publish scalar diagnostic summary facts for missing/rejected binding, invalid
  binding token, invalid residence token, invalid geometry, invalid slot shape,
  and closed-substrate rejection.
- Keep the observer read-only.

## Stop Lines

- No new arena-slot rows.
- No real raw pointer residence.
- No pointer-derived lookup or dereference.
- No real arena backing allocation.
- No real segment-map mutation.
- No real segment allocation/free execution.
- No atomic bitmap execution.
- No OSVM/page-source execution.
- No TLS, worker-local, worker scheduling, or source-level concurrency.
- No provider activation, host allocator replacement, hooks, or
  `#[global_allocator]`.
- No cross-function `Result` direct ABI or runtime sum materialization.
- No backend `.inc` matcher by app or owner name.

## Required Evidence

```text
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_arena_slot_diagnostics_guard.sh --level L0
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_arena_slot_diagnostics_guard.sh --level L1
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_arena_slot_diagnostics_guard.sh --level L2
bash tools/checks/run_proof_app.sh --only MIMAP-257A
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Landed Scope

MIMAP-257A added the modeled arena-slot diagnostic owner, proof app, SSOT,
manifest entry, check index entry, and L2 guard:

```text
lang/src/hako_alloc/memory/segment_arena_backing_modeled_arena_slot_diagnostic_box.hako
apps/hako-alloc-segment-arena-backing-modeled-arena-slot-diagnostics-proof/
docs/development/current/main/design/hako-alloc-segment-arena-backing-modeled-arena-slot-diagnostics-ssot.md
tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_arena_slot_diagnostics_guard.sh
```

## Selected Next Row

MIMAP-257A selects:

```text
MIMAP-258A segment arena backing modeled arena slot closeout pack
```

MIMAP-258A should close out the MIMAP-256A / MIMAP-257A modeled arena-slot
family with representative exact-MIR L3 evidence before any real raw pointer
residence, pointer-derived lookup, real arena backing allocation, real
segment-map execution, atomic bitmap execution, OSVM/page-source execution,
worker/provider activation, cross-function `Result` direct ABI, runtime sum
materialization, or backend matcher row opens.
