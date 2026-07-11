# 3045 - MIRBUILDER-PROGRAMJSON-LOCAL-FLOAT-LITERAL-SHAPE-SCAN-CAPABILITY-001

Status: landed

## Scope

Add a narrow `.hako` ProgramJSON traversal owner for covered `Local.expr`
`Float` literal shapes.

This card moves one more local initializer token snapshot away from Rust
ASTNode projection. It reads only the ProgramJSON node type and raw value token
for shape classification; it does not claim float lowering, dynamic numeric
typing, route selection, MIR mutation, backend lowering, ID allocation,
ProgramJSON full parser support, HakoAdoption, or Source Selfhost.

## Owner

```text
ProgramJsonLocalFloatLiteralShapeScanV1
```

Implementation:

```text
lang/src/compiler/mirbuilder/program_json_local_float_literal_shape_scan.hako
```

Input:

```text
ProgramJSON v0
```

Output:

```text
LocalFloatLiteralShapeSnapshotV1
```

## Covered Rows

```text
top_local_float_zero
top_local_float_one_point_five
top_local_float_negative
top_local_float_other
top_local_int_unsupported
top_local_str_unsupported
if_then_local_float
if_else_local_float
first_stmt_not_local_unsupported
```

## Guard

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_local_float_literal_shape_scan_parity_gate.sh
```

Expected contract:

```text
owner=ProgramJsonLocalFloatLiteralShapeScanV1
output_contract=LocalFloatLiteralShapeSnapshotV1
parity_rows=9
aot_execution_status=green
programjson_traversal_used=1
string_only_facade=0
rust_astnode_projector_retire_candidate=1
float_lowering=0
dynamic_numeric_typing=0
source_selfhost_claim=0
```

## Non-Claims

```text
float_lowering=0
dynamic_numeric_typing=0
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
MIRBUILDER-PROGRAMJSON-LOCAL-FLOAT-LITERAL-SHAPE-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001
```
