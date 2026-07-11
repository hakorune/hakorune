# 3043 - MIRBUILDER-PROGRAMJSON-LOCAL-NULL-LITERAL-SHAPE-SCAN-CAPABILITY-001

Status: landed

## Scope

Add a narrow `.hako` ProgramJSON traversal owner for covered `Local.expr`
`Null` literal shapes.

This card moves one more local initializer token snapshot away from Rust
ASTNode projection. It does not claim null lowering, option semantics, route
selection, MIR mutation, backend lowering, ID allocation, ProgramJSON full
parser support, HakoAdoption, or Source Selfhost.

## Owner

```text
ProgramJsonLocalNullLiteralShapeScanV1
```

Implementation:

```text
lang/src/compiler/mirbuilder/program_json_local_null_literal_shape_scan.hako
```

Input:

```text
ProgramJSON v0
```

Output:

```text
LocalNullLiteralShapeSnapshotV1
```

## Covered Rows

```text
top_local_null_explicit
top_local_null_declared_box
top_local_null_no_initializer_projection
top_local_int_unsupported
top_local_str_unsupported
if_then_local_null
if_else_local_null
if_then_return_unsupported
first_stmt_not_local_unsupported
```

## Guard

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_local_null_literal_shape_scan_parity_gate.sh
```

Expected contract:

```text
owner=ProgramJsonLocalNullLiteralShapeScanV1
output_contract=LocalNullLiteralShapeSnapshotV1
parity_rows=9
aot_execution_status=green
programjson_traversal_used=1
string_only_facade=0
rust_astnode_projector_retire_candidate=1
null_lowering=0
option_semantics=0
source_selfhost_claim=0
```

## Non-Claims

```text
null_lowering=0
option_semantics=0
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
MIRBUILDER-PROGRAMJSON-LOCAL-NULL-LITERAL-SHAPE-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001
```
