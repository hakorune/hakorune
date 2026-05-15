# 293x-414 DOCS-SLIM-007 Lifecycle Ladder Resolver

Status: landed
Date: 2026-05-15

## Decision

Adopt the phase-card resolver helper in the lifecycle birth / parser birth /
reuse lifecycle ladder guards.

This row removes direct numbered phase-card paths from the lifecycle ladder
guards without moving cards or changing lifecycle, parser, or allocator
semantics.

## TODO

- [x] Convert `LIFECYCLE-BIRTH-001` guard card paths to
  `guard_require_phase293x_card`.
- [x] Convert `PARSER-BIRTH-001` guard card paths to
  `guard_require_phase293x_card`.
- [x] Convert `PARSER-BIRTH-002` guard card paths to
  `guard_require_phase293x_card`.
- [x] Convert `REUSE-LIFECYCLE-001` guard card paths to
  `guard_require_phase293x_card`.
- [x] Add a guard proving the converted scripts no longer contain direct
  numbered phase-293x card paths.

## Scope

- Guard path cleanup only.
- Lifecycle birth / parser birth / reuse lifecycle ladder only.
- No card movement.

## Stop Lines

- Do not move numbered cards in this row.
- Do not convert unrelated direct-reference guards in this row.
- Do not change lifecycle, parser, hako_alloc, or taskboard semantics in this
  row.
- Do not wire the resolver helper into `dev_gate.sh` or allocator-wide.

## Required Evidence

```text
bash tools/checks/docs_slim_007_lifecycle_ladder_resolver_guard.sh
bash tools/checks/docs_slim_006_m10c_runtime_decl_resolver_guard.sh
bash tools/checks/current_state_pointer_guard.sh
```

## Implementation

- Converted `k2_wide_lifecycle_birth_new_only_guard.sh`.
- Converted `k2_wide_parser_birth_direct_call_guard.sh`.
- Converted `k2_wide_parser_birth_diagnostic_hint_guard.sh`.
- Converted `k2_wide_reuse_lifecycle_explicit_methods_guard.sh`.
- Added `docs_slim_007_lifecycle_ladder_resolver_guard.sh`.

## Evidence

```text
bash tools/checks/docs_slim_007_lifecycle_ladder_resolver_guard.sh
bash tools/checks/docs_slim_006_m10c_runtime_decl_resolver_guard.sh
bash tools/checks/current_state_pointer_guard.sh
```
