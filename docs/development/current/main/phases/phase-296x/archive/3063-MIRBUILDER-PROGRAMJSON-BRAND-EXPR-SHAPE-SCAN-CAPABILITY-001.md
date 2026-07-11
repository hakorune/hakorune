# 3063 - MIRBUILDER-PROGRAMJSON-BRAND-EXPR-SHAPE-SCAN-CAPABILITY-001

Status: landed

## Scope

Add a narrow `.hako` ProgramJSON traversal owner for covered `BrandConstruct`
and `BrandUnwrap` expression shapes.

This card moves one more Brand expression token snapshot away from Rust ASTNode
projection. It reads `brand`, `underlying_type`, and `value` structurally, but
does not claim brand lowering, brand runtime semantics, route selection, MIR
mutation, backend lowering, ID allocation, ProgramJSON full parser support,
HakoAdoption, or Source Selfhost.

## Owner

```text
ProgramJsonBrandExprShapeScanV1
```

Implementation:

```text
lang/src/compiler/mirbuilder/program_json_brand_expr_shape_scan.hako
```

Input:

```text
ProgramJSON v0
```

Output:

```text
BrandExprShapeSnapshotV1
```

## Covered Rows

```text
local_construct_i64_from_int
local_construct_string_from_str
local_construct_bool_from_bool
local_construct_from_var
local_unwrap_i64_from_var
local_unwrap_string_from_var
expr_unwrap_bool_from_var
if_then_construct_i64_from_int
if_else_unwrap_i64_from_var
first_stmt_return_unsupported
```

## Guard

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_brand_expr_shape_scan_parity_gate.sh
```

Expected contract:

```text
owner=ProgramJsonBrandExprShapeScanV1
output_contract=BrandExprShapeSnapshotV1
parity_rows=10
aot_execution_status=green
programjson_traversal_used=1
string_only_facade=0
rust_astnode_projector_retire_candidate=1
brand_lowering=0
brand_runtime_semantics=0
source_selfhost_claim=0
```

## Non-Claims

```text
brand_lowering=0
brand_runtime_semantics=0
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
MIRBUILDER-PROGRAMJSON-BRAND-EXPR-SHAPE-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001
```
