# 3071 - MIRBUILDER-PROGRAMJSON-EXPR-TERNARY-SHAPE-SCAN-CAPABILITY-001

Status: landed

## Scope

Implement `ProgramJsonExprTernaryShapeScanV1` as the next ProgramJSON traversal
capability.

The owner consumes ProgramJSON structure and emits an
`ExprTernaryShapeSnapshotV1` token snapshot for covered expression-statement
ternary expression shapes.

## Minimum Rows

```text
expr_ternary_bool_int_int
expr_ternary_var_var_var
expr_ternary_compare_int_int
expr_ternary_var_str_str
expr_ternary_array_else_unsupported
if_then_expr_ternary_bool_int_int
if_else_expr_ternary_var_var_var
expr_binary_unsupported
first_stmt_not_expr_unsupported
```

## Required Output

```text
snapshot_kind=ExprTernaryShapeSnapshotV1
top_expr_ternary_shape_kind=...
if_then_expr_ternary_shape_kind=...
if_else_expr_ternary_shape_kind=...
supported_expr_ternary_count=...
unsupported_expr_ternary_count=...
```

## Acceptance

- `.hako` implementation traverses ProgramJSON object fields for covered
  `Expr.expr` ternary positions;
- parity gate compares canonical fields against Rust ASTNode-token oracle rows;
- unsupported ternary operand shapes are reported with a stable token;
- the card can name a concrete expr-ternary Rust ASTNode projector slice as
  retire-candidate after parity is green.

## Evidence

```text
bash tools/checks/rust_lifecycle_mirbuilder_programjson_expr_ternary_shape_scan_parity_gate.sh
```

Result:

```text
owner=ProgramJsonExprTernaryShapeScanV1
output_contract=ExprTernaryShapeSnapshotV1
parity_rows=9
execution_backend=aot
aot_execution_status=green
token_snapshot_parity=green
programjson_traversal_used=1
string_only_facade=0
rust_astnode_projector_retire_candidate=1
ternary_select_lowering_semantics=0
```

Implementation:

```text
lang/src/compiler/mirbuilder/program_json_expr_ternary_shape_scan.hako
```

Fixture:

```text
docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-expr-ternary-shape-scan-parity-v0.json
```

Next:

```text
MIRBUILDER-PROGRAMJSON-EXPR-TERNARY-SHAPE-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001
```

## Forbidden

- prebuilt token snapshot input;
- source string / regex / raw contains proof;
- ternary Select lowering semantics;
- RecipeMatcher execution;
- MIR mutation, backend lowering, route selection, ID allocation, or new ABI;
- full Rust ASTNode projector retirement, ProgramJSON full parser claim,
  HakoAdoption, or Source Selfhost claim.
