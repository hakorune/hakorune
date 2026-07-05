# 3061 - MIRBUILDER-PROGRAMJSON-LOCAL-MATCH-EXPR-SHAPE-SCAN-CAPABILITY-001

Status: landed

## Scope

Add a narrow `.hako` ProgramJSON traversal owner for covered `Local.expr`
`Match` shapes.

This card moves one more Local Match token snapshot away from Rust ASTNode
projection. It reads `scrutinee`, `arms`, and `else` structurally, but does not
claim match lowering, branch execution semantics, route selection, MIR mutation,
backend lowering, ID allocation, ProgramJSON full parser support, HakoAdoption,
or Source Selfhost.

## Owner

```text
ProgramJsonLocalMatchExprShapeScanV1
```

Implementation:

```text
lang/src/compiler/mirbuilder/program_json_local_match_expr_shape_scan.hako
```

Input:

```text
ProgramJSON v0
```

Output:

```text
LocalMatchExprShapeSnapshotV1
```

## Covered Rows

```text
top_var_one_int_arm_else_int
top_int_one_str_arm_else_str
top_one_arm_else_value
top_var_two_int_arms_else_int
top_var_bool_arms_else_bool
top_two_arms_else_value
top_three_arms_unsupported
if_then_var_one_int_arm_else_int
if_else_int_one_str_arm_else_str
first_stmt_return_unsupported
```

## Guard

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_local_match_expr_shape_scan_parity_gate.sh
```

Expected contract:

```text
owner=ProgramJsonLocalMatchExprShapeScanV1
output_contract=LocalMatchExprShapeSnapshotV1
parity_rows=10
aot_execution_status=green
programjson_traversal_used=1
string_only_facade=0
rust_astnode_projector_retire_candidate=1
match_lowering=0
branch_execution_semantics=0
source_selfhost_claim=0
```

## Non-Claims

```text
match_lowering=0
branch_execution_semantics=0
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
MIRBUILDER-PROGRAMJSON-LOCAL-MATCH-EXPR-SHAPE-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001
```
