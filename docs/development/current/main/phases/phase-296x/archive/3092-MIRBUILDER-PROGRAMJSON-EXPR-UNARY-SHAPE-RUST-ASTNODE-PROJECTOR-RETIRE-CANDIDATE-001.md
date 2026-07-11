# 3092 - MIRBUILDER-PROGRAMJSON-EXPR-UNARY-SHAPE-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001

Status: landed

## Scope

After 3091 is green, mark only the covered `ExprUnaryShapeSnapshotV1`
ProgramJSON traversal rows as a Rust ASTNode projector retire-candidate.

## Requires

```text
bash tools/checks/rust_lifecycle_mirbuilder_programjson_expr_unary_shape_scan_parity_gate.sh
```

## Covered Rows

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

## Retire Candidate

```text
ExprUnaryShapeSnapshotV1 for covered ProgramJSON Expr.expr Unary rows
```

## Not Retired

- full Rust ASTNode projector;
- full UnaryOp extractor or lowerer;
- unary lowering;
- RecipeMatcher, route selection, MIR lowering, MIR mutation, or ID allocation;
- ProgramJSON full parser;
- HakoAdoption for a full owner;
- Source Selfhost.

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_expr_unary_shape_retire_rust_astnode_projector_candidate_guard.sh
```

Decision fixture:

```text
docs/development/current/main/design/fixtures/rust-lifecycle/
  mirbuilder-programjson-expr-unary-shape-retire-rust-astnode-projector-candidate-v0.json
```

## Next

```text
MIRBUILDER-PROGRAMJSON-CAPABILITY-BATCH-CONTINUATION
```
