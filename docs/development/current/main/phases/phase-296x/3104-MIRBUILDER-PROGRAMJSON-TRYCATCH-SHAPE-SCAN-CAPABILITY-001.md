# 3104 - MIRBUILDER-PROGRAMJSON-TRYCATCH-SHAPE-SCAN-CAPABILITY-001

Status: landed

## Scope

Add a real `.hako` ProgramJSON traversal capability for covered top-level
`TryCatch` shapes emitted with `try`, `catch`, and `cleanup` fields.

Owner:

```text
ProgramJsonTryCatchShapeScanV1
```

Output:

```text
TryCatchShapeSnapshotV1
```

## Covered Rows

```text
try_throw_no_catch_no_cleanup
try_return_one_catch_no_cleanup
try_expr_one_catch_cleanup_expr
try_empty_many_catches_no_cleanup
try_return_no_catch_cleanup_return
nested_trycatch_unsupported
cleanup_scalar_unsupported
first_stmt_local_unsupported
```

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_trycatch_shape_scan_parity_gate.sh
```

Green output:

```text
owner=ProgramJsonTryCatchShapeScanV1
parity_rows=8
execution_backend=aot
token_snapshot_parity=green
programjson_traversal_used=1
string_only_facade=0
rust_astnode_projector_retire_candidate=1
```

## Non-Claims

```text
exception_runtime_semantics = 0
catch_matching = 0
cleanup_execution_semantics = 0
full_recipe_matcher_execution = 0
route_selection = 0
mir_mutation = 0
backend_lowering = 0
id_allocation = 0
programjson_full_parser_claim = 0
hako_adopted_for_full_owner = 0
source_selfhost_claim = 0
new_backend_route = 0
new_abi = 0
```

## Next

```text
MIRBUILDER-PROGRAMJSON-TRYCATCH-SHAPE-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001
```
