# 3047 - MIRBUILDER-PROGRAMJSON-LOCAL-RECORD-LITERAL-SHAPE-SCAN-CAPABILITY-001

Status: landed

## Scope

Add a narrow `.hako` ProgramJSON traversal owner for covered `Local.expr`
`RecordLiteral` shapes.

This card moves one more local initializer token snapshot away from Rust
ASTNode projection. It reads the record name, `fields` array, and covered field
value node kinds only. It does not claim record lowering, field layout
semantics, route selection, MIR mutation, backend lowering, ID allocation,
ProgramJSON full parser support, HakoAdoption, or Source Selfhost.

## Owner

```text
ProgramJsonLocalRecordLiteralShapeScanV1
```

Implementation:

```text
lang/src/compiler/mirbuilder/program_json_local_record_literal_shape_scan.hako
```

Input:

```text
ProgramJSON v0
```

Output:

```text
LocalRecordLiteralShapeSnapshotV1
```

## Covered Rows

```text
top_local_point_no_fields
top_local_point_one_int_field
top_local_point_two_int_fields
top_local_config_str_bool_fields
top_local_record_three_fields_unsupported
top_local_int_unsupported
if_then_local_record_point_one_int_field
if_else_local_record_config_str_field
first_stmt_not_local_unsupported
```

## Guard

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_local_record_literal_shape_scan_parity_gate.sh
```

Expected contract:

```text
owner=ProgramJsonLocalRecordLiteralShapeScanV1
output_contract=LocalRecordLiteralShapeSnapshotV1
parity_rows=9
aot_execution_status=green
programjson_traversal_used=1
string_only_facade=0
rust_astnode_projector_retire_candidate=1
record_lowering=0
field_layout_semantics=0
source_selfhost_claim=0
```

## Non-Claims

```text
record_lowering=0
field_layout_semantics=0
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
MIRBUILDER-PROGRAMJSON-LOCAL-RECORD-LITERAL-SHAPE-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001
```
