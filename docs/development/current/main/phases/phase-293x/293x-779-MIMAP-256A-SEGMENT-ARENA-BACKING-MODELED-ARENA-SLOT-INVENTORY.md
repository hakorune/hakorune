# 293x-779 MIMAP-256A Segment Arena Backing Modeled Arena Slot Inventory

Status: selected current
Date: 2026-05-19

## Decision

Add a scalar/model inventory row that records a modeled arena slot from an
accepted segment arena backing modeled residence arena-binding report.

## Context

MIMAP-252A binds accepted modeled no-escape address residence to accepted
scalar requirement matrix facts. MIMAP-253A adds diagnostics, MIMAP-254A closes
out the family with representative exact-MIR evidence, and MIMAP-255A selects
this row. The next safe bridge toward arena backing is to model the slot that
would be used by a future arena backing owner, without allocating or exposing
real backing memory.

## Scope

- Add an owner for modeled arena-slot inventory.
- Accept only an accepted modeled residence arena-binding report.
- Preserve segment id, arena id, residence token, binding token, lifetime
  generation, address alignment, requested bytes, padded bytes, and scalar slot
  facts.
- Reject missing/rejected binding reports, invalid binding/residence tokens,
  invalid geometry, invalid slot shape, and closed-substrate requirement flags.

## Stop Lines

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
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_arena_slot_guard.sh --level L0
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_arena_slot_guard.sh --level L1
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_arena_slot_guard.sh --level L2
bash tools/checks/run_proof_app.sh --only MIMAP-256A
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

MIMAP-256A uses daily L2 validation. Representative L3 evidence is deferred to
the future modeled arena-slot closeout pack unless this row opens a new backend
route shape.
