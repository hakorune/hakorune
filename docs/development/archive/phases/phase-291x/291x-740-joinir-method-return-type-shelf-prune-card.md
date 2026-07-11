# 291x-740 JoinIR Method Return Type Shelf Prune Card

Status: Landed
Date: 2026-04-30
Lane: phase-291x CoreBox surface contract cleanup

## Scope

- `src/mir/join_ir/lowering/type_inference.rs`
- `docs/development/current/main/CURRENT_STATE.toml`

## Why

`join_ir::lowering::type_inference::infer_method_return_type` had no live
caller. The active method-return type vocabulary is owned by the MIR builder
type annotation path (`src/mir/builder/types/annotation.rs`) and its
`infer_known_method_return_type` re-export. Keeping a second JoinIR-side helper
with the same role invited policy drift.

`infer_box_type` remains live through the JoinIR AST lowerer and is kept.

## Decision

Delete the unused JoinIR method-return inference helper and its local tests.
Leave constructor/box type inference intact.

## Changes

- Removed `infer_method_return_type`.
- Removed helper-only CoreBox/method return mapping code.
- Removed local tests that exercised only the deleted helper.
- Advanced `CURRENT_STATE.toml` to 291x-740.

## Proof

- `rg -n "join_ir::lowering::type_inference::infer_method_return_type|infer_method_return_type\\(&MirType|core_box_id_for_receiver|mir_type_from_return_name" src tests -g '*.rs'`
- `cargo fmt --check`
- `bash tools/checks/current_state_pointer_guard.sh`
- `git diff --check`
- `cargo test --lib --no-run`
- `cargo build --release --bin hakorune`
- `tools/checks/dev_gate.sh quick`
