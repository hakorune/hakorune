# 293x-623 MIMAP-123A Post-Same-Module-Global-Call Row Selection

Status: landed
Date: 2026-05-18

## Decision

`MIMAP-123A` is the planning row selected by
`PURE-FIRST-GLOBAL-CALL-001`.

The pure-first compiler sidecar is closed. Allocator rows can continue without
source-inlining same-module static helpers that return scalar or typed object
values.

This row should select exactly one next allocator behavior, closeout,
substrate, or narrow Hakorune acceptance row using the mimalloc validation
cadence.

Selected row:

```text
ROUTE-FIXPOINT-001
  route refresh fixpoint owner extraction
```

Rationale:

```text
PURE-FIRST-GLOBAL-CALL-001 exposed that global-call routes, user-box method
routes, generic-method routes, route-published value types, and body-supported
checks are already a route convergence system.

The next cleanup should make that owner explicit before adding more allocator
rows or more same-module helper route profiles.
```

## Scope

- Review `PURE-FIRST-GLOBAL-CALL-001` evidence.
- Apply
  `docs/development/current/main/design/mimalloc-row-validation-cadence-ssot.md`.
- Select exactly one next row.

## Stop Lines

- No new `.hako` behavior.
- No cleanup bundle.
- No source-level syntax change.
- No raw pointer residence.
- No segment-map pointer membership or lookup.
- No arena backing allocation.
- No atomic bitmap execution.
- No page-source or OSVM execution.
- No real thread scheduling.
- No worker spawning.
- No provider activation, hook, host allocator replacement, or
  `#[global_allocator]`.
- No backend `.inc` app/name matcher.

## Planned Tasks

| Step | Task | Accept | Stop line |
| --- | --- | --- | --- |
| `123A.1` | Review current evidence after PURE-FIRST-GLOBAL-CALL-001. | selected row cites validation level. | no behavior |
| `123A.2` | Pick exactly one next row. | new selected card exists. | no bundle |
| `123A.3` | Update current pointers. | current pointer guard passes. | no implementation |

## Required Evidence

```text
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Landed Result

`MIMAP-123A` selected `ROUTE-FIXPOINT-001`.

The selected row is intentionally BoxShape-only:

- no new route vocabulary
- no allocator behavior
- no source syntax
- no backend matcher
- no preflight reason expansion
- only ownership/entry cleanup for module route convergence
