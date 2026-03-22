# StepTree (`src/mir/control_tree/step_tree/`)

StepTree is the structure layer for control-flow observation.

## Read First

1. `src/mir/control_tree/README.md`
2. `mod.rs`
3. `types.rs`
4. `fact_extractor.rs`
5. `builder.rs`
6. `signature.rs`
7. `summary.rs`

## Boundaries

- This subtree observes shape; it does not emit MIR or JoinIR.
- Facts extraction and formatting should stay local, with contract decisions outside this layer.
- Tests remain local to the step-tree shape contract.

## Responsibilities

- collect AST shape facts
- derive StepTree signatures
- expose the minimal structure used by the contract box

