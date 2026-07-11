# 3051 - MIRBUILDER-PROGRAMJSON-EXIT-MARKER-SHAPE-SCAN-CAPABILITY-001

Status: landed

## Scope

Add a narrow `.hako` ProgramJSON traversal owner for covered `Break` and
`Continue` statement marker shapes.

This card moves one more exit-marker token snapshot away from Rust ASTNode
projection. It reads top-level markers, `Loop.body[0]`, and `If.then/else[0]`
only. It does not claim CFG construction, exit lowering, route selection, MIR
mutation, backend lowering, ID allocation, ProgramJSON full parser support,
HakoAdoption, or Source Selfhost.

## Owner

```text
ProgramJsonExitMarkerShapeScanV1
```

Implementation:

```text
lang/src/compiler/mirbuilder/program_json_exit_marker_shape_scan.hako
```

Input:

```text
ProgramJSON v0
```

Output:

```text
ExitMarkerShapeSnapshotV1
```

## Covered Rows

```text
top_break
top_continue
top_return_unsupported
loop_body_break
loop_body_continue
loop_body_local_unsupported
if_then_break
if_else_continue
if_then_continue_else_break
first_stmt_local_unsupported
```

## Guard

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_exit_marker_shape_scan_parity_gate.sh
```

Expected contract:

```text
owner=ProgramJsonExitMarkerShapeScanV1
output_contract=ExitMarkerShapeSnapshotV1
parity_rows=10
aot_execution_status=green
programjson_traversal_used=1
string_only_facade=0
rust_astnode_projector_retire_candidate=1
cfg_construction=0
exit_lowering=0
source_selfhost_claim=0
```

## Non-Claims

```text
cfg_construction=0
exit_lowering=0
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
MIRBUILDER-PROGRAMJSON-EXIT-MARKER-SHAPE-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001
```
