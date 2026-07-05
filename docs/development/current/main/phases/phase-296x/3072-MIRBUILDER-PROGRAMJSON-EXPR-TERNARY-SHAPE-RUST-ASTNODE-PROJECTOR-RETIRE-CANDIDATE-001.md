# 3072 - MIRBUILDER-PROGRAMJSON-EXPR-TERNARY-SHAPE-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001

Status: landed

## Scope

After 3071 is green, mark only the covered `ExprTernaryShapeSnapshotV1`
ProgramJSON traversal rows as a Rust ASTNode projector retire-candidate.

## Requires

```text
bash tools/checks/rust_lifecycle_mirbuilder_programjson_expr_ternary_shape_scan_parity_gate.sh
```

## Covered Rows

```text
expr_ternary_bool_int_int
expr_ternary_var_var_var
expr_ternary_compare_int_int
expr_ternary_var_str_str
expr_ternary_array_else_unsupported
if_then_expr_ternary_bool_int_int
if_else_expr_ternary_var_var_var
expr_binary_unsupported
first_stmt_not_expr_unsupported
```

## Retire Candidate

```text
ExprTernaryShapeSnapshotV1 for covered Expr.expr Ternary ProgramJSON rows
```

## Not Retired

- full Rust ASTNode projector;
- full expression ternary extractor;
- ternary Select lowering semantics;
- RecipeMatcher, route selection, MIR lowering, MIR mutation, or ID allocation;
- ProgramJSON full parser;
- HakoAdoption for a full owner;
- Source Selfhost.

## Acceptance

- retire fixture names only the covered expr-ternary rows;
- guard requires 3071 parity green;
- runtime Rust projector dependency removal remains `0`;
- Rust projector remains oracle-only;
- ternary Select lowering semantics remains unclaimed.

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_expr_ternary_shape_retire_rust_astnode_projector_candidate_guard.sh
```

Decision fixture:

```text
docs/development/current/main/design/fixtures/rust-lifecycle/
  mirbuilder-programjson-expr-ternary-shape-retire-rust-astnode-projector-candidate-v0.json
```

## Next

```text
MIRBUILDER-PROGRAMJSON-CAPABILITY-BATCH-CONTINUATION
```
