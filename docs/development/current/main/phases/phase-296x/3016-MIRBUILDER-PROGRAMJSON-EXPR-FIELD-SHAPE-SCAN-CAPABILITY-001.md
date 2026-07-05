# 3016 - MIRBUILDER-PROGRAMJSON-EXPR-FIELD-SHAPE-SCAN-CAPABILITY-001

Status: landed

## Scope

Implement `ProgramJsonExprFieldShapeScanV1` as the next ProgramJSON traversal
capability.

The owner must consume ProgramJSON structure and emit an
`ExprFieldShapeSnapshotV1` token snapshot for covered expression-statement
field access shapes.

## Minimum Rows

```text
expr_field_recv_var
expr_field_recv_new_no_args
expr_field_recv_new_int_arg
expr_field_recv_method_unsupported
expr_field_recv_call_unsupported
if_then_expr_field_recv_var
if_else_expr_field_recv_new_no_args
record_field_unsupported
first_stmt_not_expr_unsupported
```

## Required Output

```text
snapshot_kind=ExprFieldShapeSnapshotV1
top_expr_field_recv_kind=...
if_then_expr_field_recv_kind=...
if_else_expr_field_recv_kind=...
supported_expr_field_count=...
unsupported_expr_field_count=...
```

## Acceptance

- `.hako` implementation traverses ProgramJSON object fields for covered
  `Expr.expr` field access positions;
- parity gate compares canonical fields against Rust ASTNode-token oracle rows;
- unsupported receiver shapes are reported with a stable token;
- the card can name a concrete expr-field Rust ASTNode projector slice as
  retire-candidate after parity is green.

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_expr_field_shape_scan_parity_gate.sh
```

Implementation:

```text
lang/src/compiler/mirbuilder/program_json_expr_field_shape_scan.hako
```

Fixture:

```text
docs/development/current/main/design/fixtures/rust-lifecycle/
  mirbuilder-programjson-expr-field-shape-scan-parity-v0.json
```

## Next

```text
MIRBUILDER-PROGRAMJSON-EXPR-FIELD-SHAPE-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001
```

## Forbidden

- prebuilt token snapshot input;
- source string / regex / raw contains proof;
- RecipeMatcher execution;
- MIR mutation, backend lowering, route selection, ID allocation, or new ABI;
- full Rust ASTNode projector retirement, ProgramJSON full parser claim,
  HakoAdoption, or Source Selfhost claim.
