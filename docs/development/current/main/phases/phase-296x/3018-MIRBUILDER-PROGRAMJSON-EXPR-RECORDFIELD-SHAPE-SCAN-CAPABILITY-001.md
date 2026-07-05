# 3018 - MIRBUILDER-PROGRAMJSON-EXPR-RECORDFIELD-SHAPE-SCAN-CAPABILITY-001

Status: active

## Scope

Implement `ProgramJsonExprRecordFieldShapeScanV1` as the next ProgramJSON
traversal capability.

The owner must consume ProgramJSON structure and emit an
`ExprRecordFieldShapeSnapshotV1` token snapshot for covered
expression-statement record field access shapes.

## Minimum Rows

```text
expr_recordfield_recv_var
expr_recordfield_recv_new_no_args
expr_recordfield_recv_new_int_arg
expr_recordfield_recv_method_unsupported
expr_recordfield_recv_call_unsupported
if_then_expr_recordfield_recv_var
if_else_expr_recordfield_recv_new_no_args
plain_field_unsupported
first_stmt_not_expr_unsupported
```

## Required Output

```text
snapshot_kind=ExprRecordFieldShapeSnapshotV1
top_expr_recordfield_recv_kind=...
if_then_expr_recordfield_recv_kind=...
if_else_expr_recordfield_recv_kind=...
supported_expr_recordfield_count=...
unsupported_expr_recordfield_count=...
```

## Acceptance

- `.hako` implementation traverses ProgramJSON object fields for covered
  `Expr.expr` record field access positions;
- parity gate compares canonical fields against Rust ASTNode-token oracle rows;
- unsupported receiver shapes are reported with a stable token;
- the card can name a concrete expr-recordfield Rust ASTNode projector slice as
  retire-candidate after parity is green.

## Forbidden

- prebuilt token snapshot input;
- source string / regex / raw contains proof;
- RecipeMatcher execution;
- MIR mutation, backend lowering, route selection, ID allocation, or new ABI;
- full Rust ASTNode projector retirement, ProgramJSON full parser claim,
  HakoAdoption, or Source Selfhost claim.
