# 293x-670 MIMAP-150A Post Blocked Substrate Matrix Row Selection

Status: landed
Date: 2026-05-18

## Decision

Select the next single mimalloc/Hakorune row after MIMAP-149A proved the
blocked-substrate matrix.

## Owner

```text
docs/development/current/main/phases/phase-293x/
docs/development/current/main/design/mimalloc-allocator-first-task-granularity-ssot.md
docs/development/current/main/design/mimalloc-hakorune-joint-task-order-ssot.md
```

## Scope

- Read the MIMAP-149A matrix output and choose exactly one next boundary.
- Decide whether the next row should be:
  - allocator-only proof work,
  - Hakorune compiler/language acceptance work,
  - substrate/capability inventory,
  - or a park row when a boundary is too early.
- Keep the next row small enough for one proof app / one guard / one durable
  semantic slice.

## Candidate Boundaries

```text
raw pointer residence
segment-map lookup
arena backing allocation
atomic bitmap execution
OSVM execution
real thread scheduling
provider activation
real segment allocation/free execution
```

## Stop Lines

- Do not open multiple blocked substrates in this planning row.
- Do not implement real segment allocation/free.
- Do not add raw pointer residence, segment-map lookup, arena backing, atomic
  bitmap execution, OSVM execution, worker scheduling, provider activation, or
  backend matchers.
- Do not make provider activation, host allocator replacement, hooks, or
  `#[global_allocator]` a mimalloc completion prerequisite.

## Required Evidence

```text
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Selection

MIMAP-150A selects `MIMAP-151A segment-map scalar lookup boundary inventory`.

Rationale:

- MIMAP-149A showed that raw pointer residence is a hard blocker, but existing
  SSOTs keep unrestricted raw pointers parked behind a later `uses rawbuf` /
  no-escape view capability.
- Segment-map lookup can still advance safely as an explicit scalar
  `(segment_id, page_id, slice)` inventory without deriving identity from raw
  pointers.
- This keeps the next row allocator-owned and proof-first while preserving the
  raw pointer, arena backing, atomic bitmap, OSVM, thread, provider, and backend
  matcher stop lines.

## Closeout

MIMAP-150A landed as a planning row and selected MIMAP-151A.
