# 3069 - MIRBUILDER-PROGRAMJSON-CALL-TARGET-SHAPE-SCAN-CAPABILITY-001

Status: landed

## Scope

Add a narrow `.hako` ProgramJSON traversal owner for covered `Call.name` and
`Call.args` arity shapes.

This card moves one more Call token snapshot away from Rust ASTNode projection.
It reads `name` and `args` structurally, but does not claim call resolution,
dispatch selection, route selection, MIR mutation, backend lowering, ID
allocation, ProgramJSON full parser support, HakoAdoption, or Source Selfhost.

## Owner

```text
ProgramJsonCallTargetShapeScanV1
```

Implementation:

```text
lang/src/compiler/mirbuilder/program_json_call_target_shape_scan.hako
```

Input:

```text
ProgramJSON v0
```

Output:

```text
CallTargetShapeSnapshotV1
```

## Covered Rows

```text
expr_env_console_log_one_arg
local_simple_no_args
local_simple_one_arg
return_simple_two_args
local_dotted_static_no_args
local_dotted_static_two_args
local_to_i64_one_arg
local_int_to_str_one_arg
if_then_simple_one_arg
if_else_dotted_static_one_arg
```

## Guard

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_call_target_shape_scan_parity_gate.sh
```

Expected contract:

```text
owner=ProgramJsonCallTargetShapeScanV1
output_contract=CallTargetShapeSnapshotV1
parity_rows=10
aot_execution_status=green
programjson_traversal_used=1
string_only_facade=0
rust_astnode_projector_retire_candidate=1
call_resolution=0
dispatch_selection=0
source_selfhost_claim=0
```

## Non-Claims

```text
call_resolution=0
dispatch_selection=0
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
MIRBUILDER-PROGRAMJSON-CALL-TARGET-SHAPE-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001
```
