# 293x-625 ROUTE-DIAG-VOCAB-001 Route Diagnostics Vocabulary

Status: selected current
Date: 2026-05-18

## Decision

`ROUTE-DIAG-VOCAB-001` is the next BoxShape compiler cleanup row selected by
`ROUTE-FIXPOINT-001`.

The route refresh owner is now explicit. The next cleanup is to keep route
diagnostic reasons from drifting between Rust route planners, MIR JSON
metadata, Python pure-first preflight, C shim allowlists, and docs.

## Scope

- Define a route diagnostics vocabulary SSOT.
- Classify existing route/preflight reasons without changing behavior.
- Name owners for reason production and consumption.
- Keep Python / C / Rust behavior unchanged unless a purely diagnostic string
  alignment is required by the SSOT.

## Stop Lines

- No new route acceptance shape.
- No new proof vocabulary.
- No allocator behavior.
- No source-level syntax.
- No backend `.inc` app/name matcher.
- No silent fallback.
- No RouteLedger implementation in this row.

## Planned Tasks

| Step | Task | Accept | Stop line |
| --- | --- | --- | --- |
| `RDV.1` | Create route diagnostics vocabulary SSOT. | reason table lists owner, layer, producer, consumer, suggestion. | no behavior |
| `RDV.2` | Inventory existing preflight reasons. | Python reason strings map to SSOT rows. | no route widening |
| `RDV.3` | Inventory current typed route proof names. | docs state proof vs diagnostic reason boundary. | no proof changes |
| `RDV.4` | Update current pointers. | pointer guard and diff check pass. | no task bundle |

## Required Evidence

```text
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
