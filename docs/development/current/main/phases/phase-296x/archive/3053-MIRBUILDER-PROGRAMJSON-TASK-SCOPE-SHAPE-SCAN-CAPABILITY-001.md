# 3053 - MIRBUILDER-PROGRAMJSON-TASK-SCOPE-SHAPE-SCAN-CAPABILITY-001

Status: landed

## Scope

Add a narrow `.hako` ProgramJSON traversal owner for covered `TaskScope`
statement shapes.

This card moves one more TaskScope token snapshot away from Rust ASTNode
projection. It reads the `spelling` field and the first statement in `body`
only. It does not claim task lowering, concurrency semantics, route selection,
MIR mutation, backend lowering, ID allocation, ProgramJSON full parser support,
HakoAdoption, or Source Selfhost.

## Owner

```text
ProgramJsonTaskScopeShapeScanV1
```

Implementation:

```text
lang/src/compiler/mirbuilder/program_json_task_scope_shape_scan.hako
```

Input:

```text
ProgramJSON v0
```

Output:

```text
TaskScopeShapeSnapshotV1
```

## Covered Rows

```text
top_task_empty
top_task_local
top_task_return
top_task_break
top_task_continue
top_task_loop
top_task_expr
if_then_task_local
if_else_task_empty
first_stmt_local_unsupported
```

## Guard

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_task_scope_shape_scan_parity_gate.sh
```

Expected contract:

```text
owner=ProgramJsonTaskScopeShapeScanV1
output_contract=TaskScopeShapeSnapshotV1
parity_rows=10
aot_execution_status=green
programjson_traversal_used=1
string_only_facade=0
rust_astnode_projector_retire_candidate=1
task_lowering=0
concurrency_semantics=0
source_selfhost_claim=0
```

## Non-Claims

```text
task_lowering=0
concurrency_semantics=0
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
MIRBUILDER-PROGRAMJSON-TASK-SCOPE-SHAPE-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001
```
