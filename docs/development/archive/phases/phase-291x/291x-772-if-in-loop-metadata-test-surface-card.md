# 291x-772 If-In-Loop Metadata Test-Surface Card

Status: Landed
Date: 2026-04-30
Lane: phase-291x CoreBox surface contract cleanup

## Scope

- `src/mir/join_ir/frontend/ast_lowerer/analysis.rs`
- `src/tests/phase40_array_ext_filter_test.rs`
- `CURRENT_STATE.toml`

## Why

The Phase 40 if-in-loop metadata extraction helpers were production-visible but
only used by crate-local tests. They carried dead-code allowances because the
active lowering path no longer calls them.

`lower_loop_with_if_meta` had no live callers, and only historical comments
still described it as a public API.

## Decision

Keep the JSON-AST variable tracking helpers as explicit `#[cfg(test)]` surface
for the existing metadata tests. Delete the unused `lower_loop_with_if_meta`
facade and its private program-shape helpers.

## Landed

- Gated `extract_if_in_loop_modified_vars`, `extract_if_assigned_vars`,
  `extract_assigned_vars_from_body`, and `extract_assigned_vars_from_stmt` with
  `#[cfg(test)]`.
- Removed `lower_loop_with_if_meta`.
- Removed the now-private helper pair for loop body and loop-carried variable
  extraction.
- Synced Phase 40 test comments away from the removed public API wording.
- Advanced `CURRENT_STATE.toml` to this card.

## Remaining Queue Impact

This closes the three AST lowerer analysis `#[allow(dead_code)]` items.

## Proof

- `cargo test --lib --no-run`
- `bash tools/checks/current_state_pointer_guard.sh`
- `git diff --check`
