# 3099 - MIRBUILDER-PROGRAMJSON-IF-BRANCH-OCCUPANCY-SHAPE-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001

Status: landed

## Scope

Mark only the covered `IfBranchOccupancyShapeSnapshotV1` ProgramJSON traversal
rows as a scoped Rust ASTNode projector retire-candidate after 3098 parity is
green.

This does not retire the full Rust ASTNode projector and does not add If
lowering or branch recipe construction.

## Requires

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_if_branch_occupancy_shape_scan_parity_gate.sh
```

## Retire Candidate Scope

```text
IfBranchOccupancyShapeSnapshotV1
for covered ProgramJSON If branch occupancy rows
```

Covered rows:

```text
then_empty_else_null
then_one_else_null
then_empty_else_empty
then_empty_else_one
then_one_else_one
then_two_else_null
then_one_else_two
then_many_else_null
first_stmt_return_unsupported
if_else_scalar_unsupported
```

## Not Retired

```text
full Rust ASTNode projector
If lowering
branch recipe construction
RecipeMatcher
route selection
MIR lowering
MIR mutation
ID allocation
ProgramJSON full parser
HakoAdoption for a full owner
Source Selfhost
```

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_if_branch_occupancy_shape_retire_rust_astnode_projector_candidate_guard.sh
```

Guard result:

```text
decision=RetireCandidateScoped
parity_gate=green
covered_rows=10
if_lowering=0
branch_recipe_construction=0
full_astnode_projector_retired=0
```

## Next

```text
MIRBUILDER-PROGRAMJSON-CAPABILITY-BATCH-CONTINUATION
```
