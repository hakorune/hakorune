# 3091 - MIRBUILDER-PROGRAMJSON-EXPR-UNARY-SHAPE-SCAN-CAPABILITY-001

Status: landed

## Scope

Add real `.hako` ProgramJSON traversal for covered `Expr.expr Unary` shapes.
The owner reads the top-level ProgramJSON body, locates an `Expr` or first
`If.then` / `If.else` expression, then inspects `Unary` `op` and `operand`.

## Implemented Owner

```text
ProgramJsonExprUnaryShapeScanV1
```

Output:

```text
ExprUnaryShapeSnapshotV1
```

Covered shape kinds:

```text
MinusInt
NotBool
BitNotVar
WeakVar
MinusVar
NotVar
Unsupported
Missing
```

## Parity Rows

```text
top_minus_int
top_not_bool
top_bitnot_var
top_weak_var
if_then_minus_var_else_null
if_else_not_var
top_unknown_op_unsupported
first_stmt_box_unsupported
```

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_expr_unary_shape_scan_parity_gate.sh
```

Gate result:

```text
aot_execution_status=green
token_snapshot_parity=green
programjson_traversal_used=1
string_only_facade=0
rust_astnode_projector_retire_candidate=1
```

## Explicit Non-Claims

- unary lowering;
- full Rust ASTNode projector retirement;
- HakoAdoption for a full owner;
- ProgramJSON full parser;
- RecipeMatcher, route selection, MIR lowering, MIR mutation, or ID allocation;
- Source Selfhost;
- new backend route or ABI.

## Next

```text
MIRBUILDER-PROGRAMJSON-EXPR-UNARY-SHAPE-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001
```
