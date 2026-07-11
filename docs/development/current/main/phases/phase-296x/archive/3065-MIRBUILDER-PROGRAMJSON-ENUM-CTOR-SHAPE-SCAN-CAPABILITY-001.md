# 3065 - MIRBUILDER-PROGRAMJSON-ENUM-CTOR-SHAPE-SCAN-CAPABILITY-001

Status: landed

## Scope

Add a narrow `.hako` ProgramJSON traversal owner for covered `EnumCtor`
expression shapes.

This card moves one more EnumCtor token snapshot away from Rust ASTNode
projection. It reads `enum`, `variant`, `payload_type`, and `args`
structurally, but does not claim enum lowering, payload ABI materialization,
route selection, MIR mutation, backend lowering, ID allocation, ProgramJSON
full parser support, HakoAdoption, or Source Selfhost.

## Owner

```text
ProgramJsonEnumCtorShapeScanV1
```

Implementation:

```text
lang/src/compiler/mirbuilder/program_json_enum_ctor_shape_scan.hako
```

Input:

```text
ProgramJSON v0
```

Output:

```text
EnumCtorShapeSnapshotV1
```

## Covered Rows

```text
local_no_payload
local_one_int_payload
local_one_str_payload
local_one_bool_payload
local_one_var_payload
return_two_int_payloads
expr_two_var_payloads
local_compat_box_one_payload
if_then_one_int_payload
if_else_one_var_payload
```

## Guard

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_enum_ctor_shape_scan_parity_gate.sh
```

Expected contract:

```text
owner=ProgramJsonEnumCtorShapeScanV1
output_contract=EnumCtorShapeSnapshotV1
parity_rows=10
aot_execution_status=green
programjson_traversal_used=1
string_only_facade=0
rust_astnode_projector_retire_candidate=1
enum_lowering=0
payload_abi_materialization=0
source_selfhost_claim=0
```

## Non-Claims

```text
enum_lowering=0
payload_abi_materialization=0
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
MIRBUILDER-PROGRAMJSON-ENUM-CTOR-SHAPE-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001
```
