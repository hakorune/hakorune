# 291x-737 FuncScanner Append Defs Direct Lowerer Shelf Prune Card

Status: Landed
Date: 2026-04-30
Lane: phase-291x CoreBox surface contract cleanup

## Scope

- `src/mir/join_ir/lowering/mod.rs`
- `src/mir/join_ir/lowering/funcscanner_append_defs.rs`
- `src/tests/mir_joinir_funcscanner_append_defs.rs`
- `src/tests/joinir/lowering/mod.rs`
- `src/tests/joinir_json_min.rs`
- `tests/fixtures/joinir/v0_funcscanner_append_defs_min.jsonir`
- `apps/tests/funcscanner_append_defs_minimal.hako`
- `src/mir/join_ir/README.md`
- `docs/development/current/main/CURRENT_STATE.toml`

## Why

`lower_funcscanner_append_defs_to_joinir` was a Phase 27.14 fixed direct
lowerer for `FuncScannerBox._append_defs/2`. It no longer had a production
caller; remaining live references were the test module and the v0 JSON snapshot
case. The active append-defs loop shape is now owned by generic Case-A lowering
and the planner-required fixture gate (`phase29bq_funcscanner_append_defs_min`),
so keeping the old direct shelf made two lowering surfaces appear authoritative.

## Decision

Delete the direct `_append_defs/2` lowerer shelf and its dedicated snapshot/test
fixture. Keep the generic Case-A append-defs lowerer and its ValueId vocabulary.
The `JOINIR_TARGETS`/Case-A target-policy rows are intentionally left for a
separate registry reconciliation pass; this card only removes the uncalled
direct lowerer surface.

## Changes

- Removed the `funcscanner_append_defs` module and public re-export.
- Deleted the ignored/experiment direct-lowerer test module.
- Removed the v0 JSON snapshot case and fixture that depended on the direct
  lowerer.
- Removed the direct `_append_defs/2` minimal fixture that no longer had a
  caller.
- Updated the JoinIR README to point append-defs ownership at generic Case-A.
- Advanced `CURRENT_STATE.toml` to 291x-737.

## Proof

- `rg -n "lower_funcscanner_append_defs_to_joinir|funcscanner_append_defs\\.rs|mir_joinir_funcscanner_append_defs|funcscanner_append_defs_minimal|v0_funcscanner_append_defs_min|FuncscannerAppendDefsMin|FuncScannerBox\\._append_defs" src tests apps -g '*.rs' -g '*.hako'`
- `cargo fmt --check`
- `bash tools/checks/current_state_pointer_guard.sh`
- `git diff --check`
- `cargo test --lib --no-run`
- `cargo build --release --bin hakorune`
- `tools/checks/dev_gate.sh quick`
