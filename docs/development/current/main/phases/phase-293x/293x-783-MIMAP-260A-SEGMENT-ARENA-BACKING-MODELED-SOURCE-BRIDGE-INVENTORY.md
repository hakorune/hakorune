# 293x-783 MIMAP-260A Segment Arena Backing Modeled Source Bridge Inventory

Status: landed
Date: 2026-05-19

## Decision

Add a scalar/model inventory row that records the modeled backing source for an
accepted segment arena backing modeled arena-slot report.

## Context

MIMAP-256A records modeled arena-slot inventory rows and MIMAP-258A closes out
that family with representative exact-MIR evidence. The next safe bridge
toward arena backing is to describe the source that would feed a future real
arena backing owner, without allocating or exposing backing memory.

## Scope

- Add an owner for modeled backing source bridge inventory.
- Accept only an accepted modeled arena-slot report.
- Preserve segment id, arena id, arena-slot token, slot geometry, and source
  scalar facts.
- Reject missing/rejected arena-slot reports, invalid arena-slot token,
  invalid source shape, invalid geometry, and closed-substrate requirement
  flags.

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
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_source_bridge_guard.sh --level L0
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_source_bridge_guard.sh --level L1
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_source_bridge_guard.sh --level L2
bash tools/checks/run_proof_app.sh --only MIMAP-260A
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

MIMAP-260A uses daily L2 validation. Representative L3 evidence is deferred to
the future modeled source bridge closeout pack unless this row opens a new
backend route shape.

## Landed Scope

MIMAP-260A added the modeled source bridge owner, proof app, SSOT, manifest
entry, check index entry, and L2 guard:

```text
lang/src/hako_alloc/memory/segment_arena_backing_modeled_source_bridge_box.hako
apps/hako-alloc-segment-arena-backing-modeled-source-bridge-proof/
docs/development/current/main/design/hako-alloc-segment-arena-backing-modeled-source-bridge-ssot.md
tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_source_bridge_guard.sh
```

## Selected Next Row

MIMAP-260A selects:

```text
MIMAP-261A segment arena backing modeled source bridge diagnostics
```

MIMAP-261A should add observer-only diagnostics for the MIMAP-260A modeled
source bridge inventory without recording new source bridge rows or opening
real pointer residence, pointer-derived lookup, real arena backing allocation,
real segment-map execution, atomic bitmap execution, OSVM/page-source
execution, worker/provider activation, cross-function `Result` direct ABI,
runtime sum materialization, or backend matchers.
