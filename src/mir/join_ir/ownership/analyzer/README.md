# ProgramJSON Ownership Analyzer (`ownership/analyzer/`)

This sub-box owns the ProgramJSON side of ownership analysis.

## Responsibility

- traverse ProgramJSON statements and expressions
- collect scope-local reads / writes / condition-reads
- build `OwnershipPlan` for loop/function scopes

## Internal Shape

- `mod.rs`
  - thin entry + tests
- `core.rs`
  - analyzer state / scope model / plan construction
- `node_analysis.rs`
  - JSON statement / expression traversal

## Boundary

Keep this separate from:

- `../ast_analyzer/*`
  - real AST ownership analysis
- `../bridge/*`
  - lowering / validation glue
- `../types.rs`
  - pure ownership substrate re-exported from `hakorune_mir_joinir`
