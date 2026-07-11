# 293x-624 ROUTE-FIXPOINT-001 Owner Extraction

Status: landed
Date: 2026-05-18

## Decision

`ROUTE-FIXPOINT-001` is the BoxShape compiler cleanup row selected by
`MIMAP-123A`.

The route refresh sequence in `semantic_refresh.rs` is no longer just a local
ordering detail. It coordinates route facts that can depend on each other:

```text
generic_method_routes
global_call_routes
user_box_method_routes
route-published value_types
body-supported checks
return contracts
```

This row extracts a small RouteFixpoint owner so the semantic refresh entry
does not own that convergence shape directly.

It selects:

```text
ROUTE-DIAG-VOCAB-001
  route diagnostics vocabulary SSOT
```

## Scope

- Add a tracked RouteFixpoint owner SSOT.
- Move the module-level generic/global/user-box route convergence sequence
  behind one Rust entry point.
- Keep behavior unchanged.
- Keep the same bounded refresh count used by `PURE-FIRST-GLOBAL-CALL-001`.

## Stop Lines

- No new route acceptance shape.
- No new proof vocabulary.
- No preflight reason vocabulary change.
- No allocator behavior.
- No source-level syntax.
- No backend `.inc` app/name matcher.
- No silent fallback.

## Planned Tasks

| Step | Task | Accept | Stop line |
| --- | --- | --- | --- |
| `RFIX.1` | Document the RouteFixpoint owner and stop lines. | SSOT names owner, inputs, outputs, and non-goals. | no behavior |
| `RFIX.2` | Add `src/mir/route_fixpoint.rs`. | `semantic_refresh.rs` calls one owner entry. | no route widening |
| `RFIX.3` | Preserve current route refresh ordering. | focused route tests pass. | no heuristic changes |
| `RFIX.4` | Close out current pointers. | pointer guard and diff check pass. | no task bundle |

## Required Evidence

```text
bash tools/checks/current_state_pointer_guard.sh
cargo test -q global_call_route_plan
cargo test -q user_box_method_route_plan
git diff --check
```

## Landed Result

ROUTE-FIXPOINT-001 landed the explicit owner extraction:

- added `src/mir/route_fixpoint.rs`
- moved module-level generic/global/user-box route convergence out of
  `semantic_refresh.rs`
- preserved the existing bounded refresh sequence
- kept family-specific route materialization rules unchanged

Observed evidence:

```text
cargo check -q
cargo test -q global_call_route_plan
cargo test -q user_box_method_route_plan
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
