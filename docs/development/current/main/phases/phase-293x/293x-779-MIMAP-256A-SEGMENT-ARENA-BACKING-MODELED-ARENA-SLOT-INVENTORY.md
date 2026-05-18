# 293x-779 MIMAP-256A Segment Arena Backing Modeled Arena Slot Inventory

Status: landed
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

## Landed Scope

MIMAP-256A added the modeled arena-slot owner, proof app, SSOT, manifest entry,
check index entry, and L2 guard:

```text
lang/src/hako_alloc/memory/segment_arena_backing_modeled_arena_slot_box.hako
apps/hako-alloc-segment-arena-backing-modeled-arena-slot-proof/
docs/development/current/main/design/hako-alloc-segment-arena-backing-modeled-arena-slot-ssot.md
tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_arena_slot_guard.sh
```

## Selected Next Row

MIMAP-256A selects:

```text
MIMAP-257A segment arena backing modeled arena slot diagnostics
```

MIMAP-257A should add observer-only diagnostics for the MIMAP-256A modeled
arena-slot inventory without recording new slot rows or opening real pointer
residence, pointer-derived lookup, real arena backing allocation,
real segment-map execution, atomic bitmap execution, OSVM/page-source
execution, worker/provider activation, cross-function `Result` direct ABI,
runtime sum materialization, or backend matchers.
