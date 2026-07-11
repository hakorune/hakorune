# 3093 - MIRBUILDER-PROGRAMJSON-EXPR-ME-SHAPE-SCAN-CAPABILITY-001

Status: landed

## Scope

Add real `.hako` ProgramJSON traversal for covered `Expr.expr Me` shapes. The
owner reads the top-level ProgramJSON body, locates an `Expr` or first `If.then`
/ `If.else` expression, then distinguishes `kind=Me,type=Var,name=me` from a
plain `Variable` named `me`.

## Implemented Owner

```text
ProgramJsonExprMeShapeScanV1
```

Output:

```text
ExprMeShapeSnapshotV1
```

Covered shape kinds:

```text
MeVarSelf
PlainVarNamedMe
Unsupported
Missing
```

## Parity Rows

```text
top_me
top_plain_var_named_me
if_then_me_else_null
if_else_me
top_me_wrong_name_unsupported
top_me_wrong_type_unsupported
top_var_other_name_unsupported
first_stmt_box_unsupported
```

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_expr_me_shape_scan_parity_gate.sh
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

- receiver-origin resolution;
- `me` lowering;
- full Rust ASTNode projector retirement;
- HakoAdoption for a full owner;
- ProgramJSON full parser;
- RecipeMatcher, route selection, MIR lowering, MIR mutation, or ID allocation;
- Source Selfhost;
- new backend route or ABI.

## Next

```text
MIRBUILDER-PROGRAMJSON-EXPR-ME-SHAPE-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001
```
