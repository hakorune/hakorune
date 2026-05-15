# 293x-415 DOCS-SLIM-008 Recent Cleanup Resolver

Status: landed
Date: 2026-05-15

## Decision

Adopt the phase-card resolver helper in the recent cleanup guard cluster.

This row removes direct numbered phase-card paths from the recent LoopClean and
Stage1 lowering cleanup guards without moving cards or changing parser,
lowering, or compatibility semantics.

## TODO

- [x] Convert `LOOPCLEAN-005` guard card path to
  `guard_require_phase293x_card`.
- [x] Convert `LOOPCLEAN-006` guard card path to
  `guard_require_phase293x_card`.
- [x] Convert `CLEAN-STAGE1-LOWERING-002` guard card path to
  `guard_require_phase293x_card`.
- [x] Add a guard proving the converted scripts no longer contain direct
  numbered phase-293x card paths.

## Scope

- Guard path cleanup only.
- Recent cleanup guard cluster only.
- No card movement.

## Stop Lines

- Do not move numbered cards in this row.
- Do not convert unrelated direct-reference guards in this row.
- Do not change parser, Stage1 lowering, Program JSON, or compatibility
  semantics in this row.
- Do not wire the resolver helper into `dev_gate.sh` or allocator-wide.

## Required Evidence

```text
bash tools/checks/docs_slim_008_recent_cleanup_resolver_guard.sh
bash tools/checks/docs_slim_007_lifecycle_ladder_resolver_guard.sh
bash tools/checks/current_state_pointer_guard.sh
```

## Implementation

- Converted `k2_wide_looprange_ast_rename_guard.sh`.
- Converted `k2_wide_loopclean_while_parser_facade_guard.sh`.
- Converted `k2_wide_clean_stage1_lowering_stmt_split_guard.sh`.
- Added `docs_slim_008_recent_cleanup_resolver_guard.sh`.

## Evidence

```text
bash tools/checks/docs_slim_008_recent_cleanup_resolver_guard.sh
bash tools/checks/docs_slim_007_lifecycle_ladder_resolver_guard.sh
bash tools/checks/current_state_pointer_guard.sh
```
