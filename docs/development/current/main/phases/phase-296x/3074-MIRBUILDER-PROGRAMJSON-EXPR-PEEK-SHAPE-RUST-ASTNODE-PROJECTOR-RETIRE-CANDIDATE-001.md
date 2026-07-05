# 3074 - MIRBUILDER-PROGRAMJSON-EXPR-PEEK-SHAPE-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001

Status: landed

## Scope

After 3073 is green, mark only the covered `ExprPeekShapeSnapshotV1`
ProgramJSON traversal rows as a Rust ASTNode projector retire-candidate.

## Requires

```text
bash tools/checks/rust_lifecycle_mirbuilder_programjson_expr_peek_shape_scan_parity_gate.sh
```

## Covered Rows

```text
expr_peek_var_one_arm_null_else
expr_peek_var_two_arm_int_else
expr_peek_call_one_arm_str_else
expr_peek_var_no_arm_null_else
expr_peek_var_three_arm_unsupported
if_then_expr_peek_var_one_arm_null_else
if_else_expr_peek_var_two_arm_int_else
expr_ternary_unsupported
first_stmt_not_expr_unsupported
```

## Retire Candidate

```text
ExprPeekShapeSnapshotV1 for covered Expr.expr Peek ProgramJSON rows
```

## Not Retired

- full Rust ASTNode projector;
- full expression Peek extractor;
- PeekParse lowering semantics or pattern semantics;
- RecipeMatcher, route selection, MIR lowering, MIR mutation, or ID allocation;
- ProgramJSON full parser;
- HakoAdoption for a full owner;
- Source Selfhost.

## Acceptance

- retire fixture names only the covered expr-peek rows;
- guard requires 3073 parity green;
- runtime Rust projector dependency removal remains `0`;
- Rust projector remains oracle-only;
- PeekParse lowering semantics and pattern semantics remain unclaimed.

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_expr_peek_shape_retire_rust_astnode_projector_candidate_guard.sh
```

Decision fixture:

```text
docs/development/current/main/design/fixtures/rust-lifecycle/
  mirbuilder-programjson-expr-peek-shape-retire-rust-astnode-projector-candidate-v0.json
```

## Next

```text
MIRBUILDER-PROGRAMJSON-CAPABILITY-BATCH-CONTINUATION
```
