# 3057 - MIRBUILDER-PROGRAMJSON-LOCAL-RECORD-UPDATE-SHAPE-SCAN-CAPABILITY-001

Status: landed

## Scope

Add a narrow `.hako` ProgramJSON traversal owner for covered `Local.expr`
`RecordUpdate` shapes.

This card moves one more Local RecordUpdate token snapshot away from Rust
ASTNode projection. It reads `record`, `base`, and `updates` structurally, but
does not claim record lowering, field layout semantics, route selection, MIR
mutation, backend lowering, ID allocation, ProgramJSON full parser support,
HakoAdoption, or Source Selfhost.

## Owner

```text
ProgramJsonLocalRecordUpdateShapeScanV1
```

Implementation:

```text
lang/src/compiler/mirbuilder/program_json_local_record_update_shape_scan.hako
```

Input:

```text
ProgramJSON v0
```

Output:

```text
LocalRecordUpdateShapeSnapshotV1
```

## Covered Rows

```text
top_point_no_updates
top_point_one_int_update
top_config_one_str_update
top_other_one_bool_update
top_other_one_var_update
top_point_two_int_updates
top_config_str_bool_updates
if_then_point_one_int_update
if_else_point_no_updates
first_stmt_return_unsupported
```

## Guard

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_local_record_update_shape_scan_parity_gate.sh
```

Expected contract:

```text
owner=ProgramJsonLocalRecordUpdateShapeScanV1
output_contract=LocalRecordUpdateShapeSnapshotV1
parity_rows=10
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
MIRBUILDER-PROGRAMJSON-LOCAL-RECORD-UPDATE-SHAPE-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001
```
