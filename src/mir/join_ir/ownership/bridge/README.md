# Ownership Bridge (`src/mir/join_ir/ownership/bridge/`)

This sub-box contains the glue between ownership analysis and JoinIR lowering.

## Responsibility

- adapt `OwnershipPlan` into lowering inputs
- validate `OwnershipPlan` against lowering/runtime-facing carrier contracts

## Non-Goals

- collect ownership facts from AST/ProgramJSON
- define ownership substrate types
- package this subtree into a separate crate yet

## Files

- `plan_to_lowering.rs`
  - analysis-to-lowering adapter
- `plan_validator.rs`
  - reusable validator for relay/carrier/condition checks

## Boundary

Keep this box separate from:

- `../analyzer.rs`
  - ProgramJSON ownership analysis core
- `../ast_analyzer/*`
  - AST ownership analysis core
- `../types.rs`
  - pure substrate now re-exported from `hakorune_mir_joinir`
