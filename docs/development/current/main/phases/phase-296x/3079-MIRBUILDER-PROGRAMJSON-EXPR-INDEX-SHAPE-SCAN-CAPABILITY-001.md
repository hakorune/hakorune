# 3079 - MIRBUILDER-PROGRAMJSON-EXPR-INDEX-SHAPE-SCAN-CAPABILITY-001

Status: landed

## Scope

Implement `ProgramJsonExprIndexShapeScanV1` as the next ProgramJSON traversal
capability.

The owner consumes ProgramJSON structure and emits an `ExprIndexShapeSnapshotV1`
token snapshot for covered expression-statement `Index` expression shapes.

ProgramJSON v0 evidence is `src/macro/ast_json/joinir_compat.rs`, where
`ASTNode::Index` emits `type: "Index"` with `target` and `index` fields.

## Minimum Rows

```text
expr_index_target_var_index_int
expr_index_target_var_index_var
expr_index_target_field_var_index_int
expr_index_target_var_index_compare
expr_index_target_call_unsupported
expr_index_index_call_unsupported
if_then_expr_index_target_var_index_int
if_else_expr_index_target_var_index_var
first_stmt_not_expr_unsupported
```

## Required Output

```text
snapshot_kind=ExprIndexShapeSnapshotV1
top_expr_index_shape_kind=...
if_then_expr_index_shape_kind=...
if_else_expr_index_shape_kind=...
supported_expr_index_count=...
unsupported_expr_index_count=...
```

## Acceptance

- `.hako` implementation traverses ProgramJSON object fields for covered
  `Expr.expr` Index positions;
- parity gate compares canonical fields against Rust ASTNode-token oracle rows;
- unsupported Index shapes are reported with a stable token;
- `index` scanner field lookup is explicit to avoid dynamic field-name fallback
  on the AOT path;
- the card can name a concrete expr-index Rust ASTNode projector slice as
  retire-candidate after parity is green.

## Evidence

```text
bash tools/checks/rust_lifecycle_mirbuilder_programjson_expr_index_shape_scan_parity_gate.sh
```

Result:

```text
owner=ProgramJsonExprIndexShapeScanV1
output_contract=ExprIndexShapeSnapshotV1
parity_rows=9
execution_backend=aot
aot_execution_status=green
token_snapshot_parity=green
programjson_traversal_used=1
string_only_facade=0
rust_astnode_projector_retire_candidate=1
index_route_selection=0
array_get_lowering=0
```

Implementation:

```text
lang/src/compiler/mirbuilder/program_json_expr_index_shape_scan.hako
```

Fixture:

```text
docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-expr-index-shape-scan-parity-v0.json
```

Next:

```text
MIRBUILDER-PROGRAMJSON-EXPR-INDEX-SHAPE-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001
```

## Forbidden

- prebuilt token snapshot input;
- source string / regex / raw contains proof;
- Index route selection or array get/set lowering;
- RecipeMatcher execution;
- MIR mutation, backend lowering, route selection, ID allocation, or new ABI;
- full Rust ASTNode projector retirement, ProgramJSON full parser claim,
  HakoAdoption, or Source Selfhost claim.
