# 3006 - MIRBUILDER-PROGRAMJSON-RETURN-EXPR-SHAPE-SCAN-CAPABILITY-001

Status: active

## Scope

Implement `ProgramJsonReturnExprShapeScanV1` as the next ProgramJSON traversal
capability.

The owner must consume ProgramJSON structure and emit a
`ReturnExprShapeSnapshotV1` token snapshot for covered `Return.expr` shapes.

## Minimum Rows

```text
top_return_int
top_return_var
top_return_bool_true
top_return_compare_var_lt_int
top_return_compare_var_eq_int
top_return_call_unsupported
if_then_return_int
if_else_return_var
```

## Required Output

```text
snapshot_kind=ReturnExprShapeSnapshotV1
top_return_expr_kind=...
if_then_return_expr_kind=...
if_else_return_expr_kind=...
supported_return_expr_count=...
unsupported_return_expr_count=...
```

## Acceptance

- `.hako` implementation traverses ProgramJSON object fields for covered
  `Return.expr` positions;
- parity gate compares canonical fields against Rust ASTNode-token oracle rows;
- unsupported return expressions are reported with a stable token;
- the card can name a concrete return-expr Rust ASTNode projector slice as
  retire-candidate after parity is green.

## Forbidden

- prebuilt token snapshot input;
- source string / regex / raw contains proof;
- RecipeMatcher execution;
- MIR mutation, backend lowering, route selection, ID allocation, or new ABI;
- full Rust ASTNode projector retirement, ProgramJSON full parser claim,
  HakoAdoption, or Source Selfhost claim.
