# 293x-412 DOCS-SLIM-005 Production Closeout Resolver

Status: landed
Date: 2026-05-15

## Decision

Adopt the phase-card resolver helper in the production allocator port closeout
guard.

`DOCS-SLIM-004` converted the allocator-provider activation closeout family.
This row handles the older production allocator port closeout family separately
so the cleanup stays easy to review.

## TODO

- [x] Convert the M51 production allocator port closeout card reference to
  `guard_require_phase293x_card`.
- [x] Convert the M46-M50 required card references to
  `guard_require_phase293x_card`.
- [x] Add a guard proving the production closeout script no longer contains
  direct phase-293x card paths.
- [x] Keep phase README / taskboard assertions unchanged in this row.

## Scope

- Guard path cleanup only.
- Production allocator port closeout guard only.
- No card movement.

## Stop Lines

- Do not move numbered cards in this row.
- Do not convert unrelated direct-reference guards in this row.
- Do not change phase README or taskboard assertions in this row.
- Do not wire the resolver helper into `dev_gate.sh` or allocator-wide.

## Required Evidence

```text
bash tools/checks/docs_slim_005_production_closeout_resolver_guard.sh
bash tools/checks/docs_slim_004_activation_closeout_resolver_guard.sh
bash tools/checks/current_state_pointer_guard.sh
```

## Implementation

- Converted `k2_wide_production_allocator_port_closeout_guard.sh`.
- Added `docs_slim_005_production_closeout_resolver_guard.sh`.

## Evidence

```text
bash tools/checks/docs_slim_005_production_closeout_resolver_guard.sh
bash tools/checks/docs_slim_004_activation_closeout_resolver_guard.sh
bash tools/checks/current_state_pointer_guard.sh
```
