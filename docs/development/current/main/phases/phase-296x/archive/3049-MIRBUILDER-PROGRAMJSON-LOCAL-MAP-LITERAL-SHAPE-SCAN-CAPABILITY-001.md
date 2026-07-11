# 3049 - MIRBUILDER-PROGRAMJSON-LOCAL-MAP-LITERAL-SHAPE-SCAN-CAPABILITY-001

Status: landed

## Scope

Add a narrow `.hako` ProgramJSON traversal owner for covered `Local.expr`
`Map` literal shapes.

This card moves one more local initializer token snapshot away from Rust
ASTNode projection. It reads the `entries` array and covered entry value node
kinds only. It does not claim MapBox lowering, map allocation semantics, route
selection, MIR mutation, backend lowering, ID allocation, ProgramJSON full
parser support, HakoAdoption, or Source Selfhost.

## Owner

```text
ProgramJsonLocalMapLiteralShapeScanV1
```

Implementation:

```text
lang/src/compiler/mirbuilder/program_json_local_map_literal_shape_scan.hako
```

Input:

```text
ProgramJSON v0
```

Output:

```text
LocalMapLiteralShapeSnapshotV1
```

## Covered Rows

```text
top_local_map_empty
top_local_map_one_int
top_local_map_one_str
top_local_map_one_bool
top_local_map_two_int_str
top_local_map_three_entries_unsupported
top_local_int_unsupported
if_then_local_map_one_int
if_else_local_map_empty
first_stmt_not_local_unsupported
```

## Guard

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_local_map_literal_shape_scan_parity_gate.sh
```

Expected contract:

```text
owner=ProgramJsonLocalMapLiteralShapeScanV1
output_contract=LocalMapLiteralShapeSnapshotV1
parity_rows=10
aot_execution_status=green
programjson_traversal_used=1
string_only_facade=0
rust_astnode_projector_retire_candidate=1
map_lowering=0
map_allocation_semantics=0
source_selfhost_claim=0
```

## Non-Claims

```text
map_lowering=0
map_allocation_semantics=0
source_selfhost_claim=0
mir_mutation=0
id_allocation=0
backend_lowering=0
full_recipe_matcher_execution=0
route_selection=0
runtime_fallback=0
new_backend_route=0
new_abi=0
programjson_full_parser_claim=0
hako_adopted_for_full_owner=0
```

## Next

```text
MIRBUILDER-PROGRAMJSON-LOCAL-MAP-LITERAL-SHAPE-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001
```
