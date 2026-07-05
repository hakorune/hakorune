# 3026 - MIRBUILDER-PROGRAMJSON-EXPR-LOGICAL-SHAPE-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001

Status: active

## Scope

After 3025 is green, mark only the covered `ExprLogicalShapeSnapshotV1`
ProgramJSON traversal rows as a Rust ASTNode projector retire-candidate.

## Requires

```text
bash tools/checks/rust_lifecycle_mirbuilder_programjson_expr_logical_shape_scan_parity_gate.sh
```

## Covered Rows

```text
expr_logical_var_and_var
expr_logical_var_or_var
expr_logical_bool_and_var
expr_logical_var_or_bool
expr_logical_nested_unsupported
if_then_expr_logical_var_and_var
if_else_expr_logical_var_or_var
compare_expr_unsupported
first_stmt_not_expr_unsupported
```

## Retire Candidate

```text
ExprLogicalShapeSnapshotV1 for covered Expr.expr Logical ProgramJSON rows
```

## Not Retired

- full Rust ASTNode projector;
- full expression logical extractor;
- boolean short-circuit lowering semantics;
- RecipeMatcher, route selection, MIR lowering, MIR mutation, or ID allocation;
- ProgramJSON full parser;
- HakoAdoption for a full owner;
- Source Selfhost.

## Acceptance

- retire fixture names only the covered expr-logical rows;
- guard requires 3025 parity green;
- runtime Rust projector dependency removal remains `0`;
- Rust projector remains oracle-only;
- boolean short-circuit lowering semantics remains unclaimed.
