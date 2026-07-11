# 291x-734 Scan Minimal Lowerer Shelf Prune Card

Status: Landed
Date: 2026-04-30
Lane: phase-291x CoreBox surface contract cleanup

## Scope

- `src/mir/join_ir/lowering/mod.rs`
- `src/mir/join_ir/lowering/{scan_bool_predicate_minimal,scan_with_init_minimal,scan_with_init_reverse,split_scan_minimal}.rs`
- `docs/development/current/main/design/compiler-task-map-ssot.md`
- `docs/development/current/main/phases/phase-272/README.md`
- `docs/development/current/main/CURRENT_STATE.toml`

## Why

The scan family direct JoinIR lowerers had no live Rust callers outside their
own unit tests and module exports. Current scan routing is owned by the
`joinir::route_entry` registry and `RecipeComposer`/Facts path, so keeping the
old hand-written JoinIR modules made retired surface look active.

## Decision

Delete the four direct lowerer shelves and remove their module exports. Keep
historical/archive references intact, but update current design docs that still
called these files current route owners.

## Changes

- Removed the `scan_bool_predicate_minimal`, `scan_with_init_minimal`,
  `scan_with_init_reverse`, and `split_scan_minimal` module exports.
- Deleted the four hand-written JoinIR lowerer files and their self-contained
  unit tests.
- Updated current route-owner docs to point at `route_entry` registry,
  `RecipeComposer`, and Facts instead of the deleted files.
- Advanced `CURRENT_STATE.toml` to 291x-734.

## Proof

- `rg -n "lower_scan_bool_predicate_minimal|lower_scan_with_init_minimal|lower_scan_with_init_reverse|lower_split_scan_minimal" src tests tools -g '*.rs' -g '*.sh'`
- `rg -n "pub mod (scan_bool_predicate_minimal|scan_with_init_minimal|scan_with_init_reverse|split_scan_minimal)" src/mir/join_ir/lowering/mod.rs`
- `rg -n "current route file|current route files" docs/development/current/main/phases/phase-272/README.md docs/development/current/main/design/compiler-task-map-ssot.md`
- `cargo fmt --check`
- `bash tools/checks/current_state_pointer_guard.sh`
- `git diff --check`
- `cargo test --lib --no-run`
- `cargo build --release --bin hakorune`
