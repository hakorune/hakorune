# 293x-959 MIMAP-344A No-Escape Pointer Residence Pilot

Status: landed
Date: 2026-05-21

## Decision

Open the first small real seam as a no-escape pointer residence pilot.

## Context

MIMAP-342A bundled the remaining model-only release/recycle execution
requirements, and MIMAP-343A closed that prerequisite ledger. The next step can
open a narrow real seam, but it must not jump directly to pointer-derived
lookup, arena release/recycle, segment-map mutation, atomic bitmap, OSVM,
worker/TLS, or provider activation.

## Scope

- Add a no-escape pointer residence owner/proof/guard.
- Represent pointer residence as a bounded private token suitable for proof app
  scope only.
- Keep escape paths closed:
  - no return of pointer token across function boundary
  - no object field storage
  - no ArrayBox/MapBox storage
  - no pointer-derived lookup or dereference

## Stop Lines

- No real release/recycle execution.
- No pointer-derived lookup or dereference.
- No real arena backing allocation, release, or recycle.
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
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_no_escape_pointer_residence_pilot_guard.sh --level L2
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

Landed the no-escape pointer residence pilot with a private proof-scope token
and L2 VM/MIR evidence. MIMAP-345A is selected to add the next narrow seam:
an arena backing handle pilot that still keeps arena release/recycle,
pointer-derived lookup, segment-map mutation, atomic bitmap, OSVM, worker/TLS,
provider activation, and backend matcher execution closed.
