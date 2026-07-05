# 3102 - MIRBUILDER-PROGRAMJSON-BLOCK-EXPR-SHAPE-SCAN-CAPABILITY-001

Status: landed

## Scope

Add a real `.hako` ProgramJSON traversal capability for covered top-level
`BlockExpr` shapes emitted with `prelude_stmts` and `tail_expr`.

Owner:

```text
ProgramJsonBlockExprShapeScanV1
```

Output:

```text
BlockExprShapeSnapshotV1
```

## Covered Rows

```text
empty_prelude_tail_int
empty_prelude_tail_string
empty_prelude_tail_bool
local_prelude_tail_int
expr_prelude_tail_string
return_prelude_tail_var
local_expr_prelude_tail_var
many_prelude_tail_int
tail_unsupported
first_stmt_local_unsupported
```

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_block_expr_shape_scan_parity_gate.sh
```

Green output:

```text
owner=ProgramJsonBlockExprShapeScanV1
parity_rows=10
execution_backend=aot
token_snapshot_parity=green
programjson_traversal_used=1
string_only_facade=0
rust_astnode_projector_retire_candidate=1
```

## Non-Claims

```text
block_expr_lowering = 0
prelude_execution_semantics = 0
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
MIRBUILDER-PROGRAMJSON-BLOCK-EXPR-SHAPE-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001
```
