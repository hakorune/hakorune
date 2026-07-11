# 3020 - MIRBUILDER-PROGRAMJSON-EXPR-BINARY-SHAPE-SCAN-CAPABILITY-001

Status: landed

## Scope

Implement `ProgramJsonExprBinaryShapeScanV1` as the next ProgramJSON traversal
capability.

The owner must consume ProgramJSON structure and emit an
`ExprBinaryShapeSnapshotV1` token snapshot for covered expression-statement
binary expression shapes.

## Minimum Rows

```text
expr_binary_var_add_int
expr_binary_int_sub_var
expr_binary_var_mul_int
expr_binary_var_div_int
expr_binary_nested_unsupported
if_then_expr_binary_var_add_int
if_else_expr_binary_int_sub_var
compare_expr_unsupported
first_stmt_not_expr_unsupported
```

## Required Output

```text
snapshot_kind=ExprBinaryShapeSnapshotV1
top_expr_binary_shape_kind=...
if_then_expr_binary_shape_kind=...
if_else_expr_binary_shape_kind=...
supported_expr_binary_count=...
unsupported_expr_binary_count=...
```

## Acceptance

- `.hako` implementation traverses ProgramJSON object fields for covered
  `Expr.expr` binary positions;
- parity gate compares canonical fields against Rust ASTNode-token oracle rows;
- unsupported binary operand shapes are reported with a stable token;
- the card can name a concrete expr-binary Rust ASTNode projector slice as
  retire-candidate after parity is green.

## Evidence

```text
bash tools/checks/rust_lifecycle_mirbuilder_programjson_expr_binary_shape_scan_parity_gate.sh
```

Result:

```text
owner=ProgramJsonExprBinaryShapeScanV1
output_contract=ExprBinaryShapeSnapshotV1
parity_rows=9
execution_backend=aot
aot_execution_status=green
token_snapshot_parity=green
programjson_traversal_used=1
string_only_facade=0
rust_astnode_projector_retire_candidate=1
```

Implementation:

```text
lang/src/compiler/mirbuilder/program_json_expr_binary_shape_scan.hako
```

Fixture:

```text
docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-expr-binary-shape-scan-parity-v0.json
```

Next:

```text
MIRBUILDER-PROGRAMJSON-EXPR-BINARY-SHAPE-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001
```

## Forbidden

- prebuilt token snapshot input;
- source string / regex / raw contains proof;
- RecipeMatcher execution;
- MIR mutation, backend lowering, route selection, ID allocation, or new ABI;
- full Rust ASTNode projector retirement, ProgramJSON full parser claim,
  HakoAdoption, or Source Selfhost claim.
