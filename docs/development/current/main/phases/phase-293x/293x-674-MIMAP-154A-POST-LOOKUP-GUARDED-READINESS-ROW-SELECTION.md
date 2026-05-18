# 293x-674 MIMAP-154A Post Lookup Guarded Readiness Row Selection

Status: landed
Date: 2026-05-18

## Decision

Select the next single row after MIMAP-153A proved lookup-guarded membership
and allocation readiness.

Selected:

```text
MIMAP-155A segment-map readiness validation pack closeout guard
```

Rationale:

- MIMAP-149A, MIMAP-151A, and MIMAP-153A now form one explicit-ID
  segment-map readiness family.
- ROW-VALIDATION-PROFILE-001/002 added `segment-map-readiness` manifest
  metadata and L2 split commands for the family.
- Before adding another allocator behavior row, freeze the family as a small
  validation pack so later rows can rely on `--level L2` for daily work and
  reserve full EXE validation for first-pattern/backend-route/closeout evidence.

## Owner

```text
docs/development/current/main/phases/phase-293x/
docs/development/current/main/design/mimalloc-allocator-first-task-granularity-ssot.md
docs/development/current/main/design/mimalloc-hakorune-joint-task-order-ssot.md
```

## Scope

- Choose exactly one follow-up after explicit-ID lookup guarded readiness.
- Prefer the smallest row that composes the accepted readiness path into the
  existing modeled allocation consume / ledger proof lane.
- Park any follow-up that requires rawbuf, atomics, OSVM, scheduling, or
  provider substrate.

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
