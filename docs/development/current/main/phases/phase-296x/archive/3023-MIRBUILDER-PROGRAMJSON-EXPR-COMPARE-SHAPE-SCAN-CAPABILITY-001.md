# 3023 - MIRBUILDER-PROGRAMJSON-EXPR-COMPARE-SHAPE-SCAN-CAPABILITY-001

Status: landed

## Scope

Implement `ProgramJsonExprCompareShapeScanV1` as the next ProgramJSON traversal
capability.

The owner must consume ProgramJSON structure and emit an
`ExprCompareShapeSnapshotV1` token snapshot for covered expression-statement
compare expression shapes.

## Minimum Rows

```text
expr_compare_var_lt_int
expr_compare_var_eq_int
expr_compare_var_le_int
expr_compare_var_gt_int
expr_compare_var_ge_int
expr_compare_nested_unsupported
if_then_expr_compare_var_lt_int
if_else_expr_compare_var_eq_int
binary_expr_unsupported
first_stmt_not_expr_unsupported
```

## Required Output

```text
snapshot_kind=ExprCompareShapeSnapshotV1
top_expr_compare_shape_kind=...
if_then_expr_compare_shape_kind=...
if_else_expr_compare_shape_kind=...
supported_expr_compare_count=...
unsupported_expr_compare_count=...
```

## Acceptance

- `.hako` implementation traverses ProgramJSON object fields for covered
  `Expr.expr` compare positions;
- parity gate compares canonical fields against Rust ASTNode-token oracle rows;
- unsupported compare operand shapes are reported with a stable token;
- the card can name a concrete expr-compare Rust ASTNode projector slice as
  retire-candidate after parity is green.

## Evidence

```text
bash tools/checks/rust_lifecycle_mirbuilder_programjson_expr_compare_shape_scan_parity_gate.sh
```

Result:

```text
owner=ProgramJsonExprCompareShapeScanV1
output_contract=ExprCompareShapeSnapshotV1
parity_rows=10
execution_backend=aot
aot_execution_status=green
token_snapshot_parity=green
programjson_traversal_used=1
string_only_facade=0
rust_astnode_projector_retire_candidate=1
string_relational_ordering_semantics=0
```

Implementation:

```text
lang/src/compiler/mirbuilder/program_json_expr_compare_shape_scan.hako
```

Fixture:

```text
docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-expr-compare-shape-scan-parity-v0.json
```

Next:

```text
MIRBUILDER-PROGRAMJSON-EXPR-COMPARE-SHAPE-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001
```

## Forbidden

- prebuilt token snapshot input;
- source string / regex / raw contains proof;
- string relational ordering semantics decision;
- RecipeMatcher execution;
- MIR mutation, backend lowering, route selection, ID allocation, or new ABI;
- full Rust ASTNode projector retirement, ProgramJSON full parser claim,
  HakoAdoption, or Source Selfhost claim.
