# 3024 - MIRBUILDER-PROGRAMJSON-EXPR-COMPARE-SHAPE-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001

Status: landed

## Scope

After 3023 is green, mark only the covered `ExprCompareShapeSnapshotV1`
ProgramJSON traversal rows as a Rust ASTNode projector retire-candidate.

## Requires

```text
bash tools/checks/rust_lifecycle_mirbuilder_programjson_expr_compare_shape_scan_parity_gate.sh
```

## Covered Rows

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

## Retire Candidate

```text
ExprCompareShapeSnapshotV1 for covered Expr.expr Compare ProgramJSON rows
```

## Not Retired

- full Rust ASTNode projector;
- full expression compare extractor;
- string relational ordering semantics;
- RecipeMatcher, route selection, MIR lowering, MIR mutation, or ID allocation;
- ProgramJSON full parser;
- HakoAdoption for a full owner;
- Source Selfhost.

## Acceptance

- retire fixture names only the covered expr-compare rows;
- guard requires 3023 parity green;
- runtime Rust projector dependency removal remains `0`;
- Rust projector remains oracle-only;
- string relational ordering semantics remains unclaimed.

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_expr_compare_shape_retire_rust_astnode_projector_candidate_guard.sh
```

Decision fixture:

```text
docs/development/current/main/design/fixtures/rust-lifecycle/
  mirbuilder-programjson-expr-compare-shape-retire-rust-astnode-projector-candidate-v0.json
```

## Next

```text
MIRBUILDER-PROGRAMJSON-EXPR-LOGICAL-SHAPE-SCAN-CAPABILITY-001
```
