# 3098 - MIRBUILDER-PROGRAMJSON-IF-BRANCH-OCCUPANCY-SHAPE-SCAN-CAPABILITY-001

Status: landed

## Scope

Add a real `.hako` ProgramJSON traversal capability for covered top-level `If`
branch occupancy shapes.

Owner:

```text
ProgramJsonIfBranchOccupancyShapeScanV1
```

Output:

```text
IfBranchOccupancyShapeSnapshotV1
```

The traversal reads actual Program(JSON v0) structure:

```text
Program.body[0] -> If.then / If.else
```

## Covered Rows

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

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_if_branch_occupancy_shape_scan_parity_gate.sh
```

Green output:

```text
owner=ProgramJsonIfBranchOccupancyShapeScanV1
parity_rows=10
execution_backend=aot
token_snapshot_parity=green
programjson_traversal_used=1
string_only_facade=0
rust_astnode_projector_retire_candidate=1
```

## Non-Claims

```text
if_lowering = 0
branch_recipe_construction = 0
full_recipe_matcher_execution = 0
route_selection = 0
mir_mutation = 0
backend_lowering = 0
id_allocation = 0
programjson_full_parser_claim = 0
hako_adopted_for_full_owner = 0
source_selfhost_claim = 0
new_backend_route = 0
new_abi = 0
```

## Next

```text
MIRBUILDER-PROGRAMJSON-IF-BRANCH-OCCUPANCY-SHAPE-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001
```
