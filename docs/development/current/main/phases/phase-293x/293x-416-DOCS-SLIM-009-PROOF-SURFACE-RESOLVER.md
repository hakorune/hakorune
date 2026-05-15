# 293x-416 DOCS-SLIM-009 Proof Surface Resolver

Status: landed
Date: 2026-05-15

## Decision

Adopt the phase-card resolver helper in the C197-C200 proof application surface
guard cluster.

This row removes direct numbered phase-card paths from the proof surface guards
without moving cards or changing parser, syntax, proof app, or backend
semantics.

## TODO

- [x] Convert `C197` logical condition surface guard card paths to
  `guard_require_phase293x_card`.
- [x] Convert `C198` check block surface guard card paths to
  `guard_require_phase293x_card`.
- [x] Convert `C199` compound assignment surface guard card paths to
  `guard_require_phase293x_card`.
- [x] Convert `C200` guard else surface guard card paths to
  `guard_require_phase293x_card`.
- [x] Add a guard proving the converted scripts no longer contain direct
  numbered phase-293x card paths.

## Scope

- Guard path cleanup only.
- C197-C200 proof surface guard cluster only.
- No card movement.

## Stop Lines

- Do not move numbered cards in this row.
- Do not convert unrelated direct-reference guards in this row.
- Do not change parser, syntax, proof app, or backend semantics in this row.
- Do not wire the resolver helper into `dev_gate.sh` or allocator-wide.

## Required Evidence

```text
bash tools/checks/docs_slim_009_proof_surface_resolver_guard.sh
bash tools/checks/docs_slim_008_recent_cleanup_resolver_guard.sh
bash tools/checks/current_state_pointer_guard.sh
```

## Implementation

- Converted `k2_wide_logical_condition_surface_guard.sh`.
- Converted `k2_wide_check_block_surface_guard.sh`.
- Converted `k2_wide_compound_assignment_surface_guard.sh`.
- Converted `k2_wide_guard_else_surface_guard.sh`.
- Added `docs_slim_009_proof_surface_resolver_guard.sh`.

## Evidence

```text
bash tools/checks/docs_slim_009_proof_surface_resolver_guard.sh
bash tools/checks/docs_slim_008_recent_cleanup_resolver_guard.sh
bash tools/checks/current_state_pointer_guard.sh
```
