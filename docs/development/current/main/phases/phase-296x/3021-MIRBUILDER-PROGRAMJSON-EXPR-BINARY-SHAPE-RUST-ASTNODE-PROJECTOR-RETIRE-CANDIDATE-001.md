# 3021 - MIRBUILDER-PROGRAMJSON-EXPR-BINARY-SHAPE-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001

Status: pending

## Scope

After 3020 is green, mark only the covered `ExprBinaryShapeSnapshotV1`
ProgramJSON traversal rows as a Rust ASTNode projector retire-candidate.

## Requires

```text
bash tools/checks/rust_lifecycle_mirbuilder_programjson_expr_binary_shape_scan_parity_gate.sh
```

## Covered Rows

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

## Retire Candidate

```text
ExprBinaryShapeSnapshotV1 for covered Expr.expr Binary ProgramJSON rows
```

## Not Retired

- full Rust ASTNode projector;
- full expression binary extractor;
- RecipeMatcher, route selection, MIR lowering, MIR mutation, or ID allocation;
- ProgramJSON full parser;
- HakoAdoption for a full owner;
- Source Selfhost.

## Acceptance

- retire fixture names only the covered expr-binary rows;
- guard requires 3020 parity green;
- runtime Rust projector dependency removal remains `0`;
- Rust projector remains oracle-only.
