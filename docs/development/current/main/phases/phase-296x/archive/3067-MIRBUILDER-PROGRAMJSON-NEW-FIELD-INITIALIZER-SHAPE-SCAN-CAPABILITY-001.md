# 3067 - MIRBUILDER-PROGRAMJSON-NEW-FIELD-INITIALIZER-SHAPE-SCAN-CAPABILITY-001

Status: landed

## Scope

Add a narrow `.hako` ProgramJSON traversal owner for covered
`New.field_initializers` shapes.

This card moves one more New expression token snapshot away from Rust ASTNode
projection. It reads `class`, `field_initializers`, `field`, and `value`
structurally, but does not claim box field initializer lowering, object
allocation, route selection, MIR mutation, backend lowering, ID allocation,
ProgramJSON full parser support, HakoAdoption, or Source Selfhost.

## Owner

```text
ProgramJsonNewFieldInitializerShapeScanV1
```

Implementation:

```text
lang/src/compiler/mirbuilder/program_json_new_field_initializer_shape_scan.hako
```

Input:

```text
ProgramJSON v0
```

Output:

```text
NewFieldInitializerShapeSnapshotV1
```

## Covered Rows

```text
local_no_field_initializers
local_one_int_field_init
local_one_str_field_init
local_one_bool_field_init
local_one_var_field_init
expr_one_new_field_init
return_two_int_field_inits
local_two_var_field_inits
if_then_one_int_field_init
if_else_one_var_field_init
```

## Guard

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_new_field_initializer_shape_scan_parity_gate.sh
```

Expected contract:

```text
owner=ProgramJsonNewFieldInitializerShapeScanV1
output_contract=NewFieldInitializerShapeSnapshotV1
parity_rows=10
aot_execution_status=green
programjson_traversal_used=1
string_only_facade=0
rust_astnode_projector_retire_candidate=1
box_field_initializer_lowering=0
object_allocation=0
source_selfhost_claim=0
```

## Non-Claims

```text
box_field_initializer_lowering=0
object_allocation=0
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
MIRBUILDER-PROGRAMJSON-NEW-FIELD-INITIALIZER-SHAPE-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001
```
