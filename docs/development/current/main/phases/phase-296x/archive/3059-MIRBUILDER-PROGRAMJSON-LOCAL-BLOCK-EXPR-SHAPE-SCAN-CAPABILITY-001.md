# 3059 - MIRBUILDER-PROGRAMJSON-LOCAL-BLOCK-EXPR-SHAPE-SCAN-CAPABILITY-001

Status: landed

## Scope

Add a narrow `.hako` ProgramJSON traversal owner for covered `Local.expr`
`BlockExpr` shapes.

This card moves one more Local BlockExpr token snapshot away from Rust ASTNode
projection. It reads `prelude` and `tail` structurally, but does not claim block
expression lowering, prelude execution semantics, route selection, MIR mutation,
backend lowering, ID allocation, ProgramJSON full parser support, HakoAdoption,
or Source Selfhost.

## Owner

```text
ProgramJsonLocalBlockExprShapeScanV1
```

Implementation:

```text
lang/src/compiler/mirbuilder/program_json_local_block_expr_shape_scan.hako
```

Input:

```text
ProgramJSON v0
```

Output:

```text
LocalBlockExprShapeSnapshotV1
```

## Covered Rows

```text
top_empty_prelude_tail_int
top_empty_prelude_tail_str
top_empty_prelude_tail_bool
top_local_prelude_tail_int
top_expr_prelude_tail_str
top_return_prelude_tail_var
top_local_expr_prelude_tail_var
if_then_local_prelude_tail_int
if_else_empty_prelude_tail_int
first_stmt_return_unsupported
```

## Guard

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_local_block_expr_shape_scan_parity_gate.sh
```

Expected contract:

```text
owner=ProgramJsonLocalBlockExprShapeScanV1
output_contract=LocalBlockExprShapeSnapshotV1
parity_rows=10
aot_execution_status=green
programjson_traversal_used=1
string_only_facade=0
rust_astnode_projector_retire_candidate=1
block_expr_lowering=0
prelude_execution_semantics=0
source_selfhost_claim=0
```

## Non-Claims

```text
block_expr_lowering=0
prelude_execution_semantics=0
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
MIRBUILDER-PROGRAMJSON-LOCAL-BLOCK-EXPR-SHAPE-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001
```
