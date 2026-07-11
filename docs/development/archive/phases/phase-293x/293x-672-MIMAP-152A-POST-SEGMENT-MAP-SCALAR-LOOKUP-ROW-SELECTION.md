# 293x-672 MIMAP-152A Post Segment Map Scalar Lookup Row Selection

Status: landed
Date: 2026-05-18

## Decision

Select the next single row after MIMAP-151A proved the explicit-ID
segment-map scalar lookup boundary.

## Owner

```text
docs/development/current/main/phases/phase-293x/
docs/development/current/main/design/mimalloc-allocator-first-task-granularity-ssot.md
docs/development/current/main/design/mimalloc-hakorune-joint-task-order-ssot.md
```

## Scope

- Choose exactly one follow-up boundary after explicit-ID segment lookup.
- Prefer the smallest row that either composes the lookup into an allocator
  proof path or parks a boundary that requires rawbuf, atomics, OSVM, or
  scheduling substrate.
- Keep the next row proof-first and guard-owned.

## Stop Lines

- Do not implement real segment allocation/free in this planning row.
- Do not open raw pointer residence, real segment-map execution, arena backing,
  atomic bitmap execution, OSVM execution, thread scheduling, provider
  activation, host allocator replacement, hooks, `#[global_allocator]`, or
  backend matchers.

## Required Evidence

```text
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Selection

MIMAP-152A selects `MIMAP-153A segment-map lookup guarded readiness
composition`.

Rationale:

- MIMAP-151A proved explicit-ID segment/page/slice lookup without raw pointer
  residence.
- The smallest useful follow-up is to gate the already-landed
  segment/page-membership and allocation-readiness scalar facts behind that
  lookup result.
- This composes existing allocator proof owners while keeping real segment-map
  execution, raw pointer residence, arena backing, atomic bitmap, OSVM, thread,
  provider, and backend matcher surfaces closed.

## Closeout

MIMAP-152A landed as a planning row and selected MIMAP-153A.
