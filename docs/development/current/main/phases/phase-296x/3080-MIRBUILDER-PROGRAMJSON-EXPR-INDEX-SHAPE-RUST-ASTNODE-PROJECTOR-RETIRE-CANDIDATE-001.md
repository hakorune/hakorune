# 3080 - MIRBUILDER-PROGRAMJSON-EXPR-INDEX-SHAPE-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001

Status: landed

## Scope

After 3079 is green, mark only the covered `ExprIndexShapeSnapshotV1`
ProgramJSON traversal rows as a Rust ASTNode projector retire-candidate.

## Requires

```text
bash tools/checks/rust_lifecycle_mirbuilder_programjson_expr_index_shape_scan_parity_gate.sh
```

## Covered Rows

```text
expr_index_target_var_index_int
expr_index_target_var_index_var
expr_index_target_field_var_index_int
expr_index_target_var_index_compare
expr_index_target_call_unsupported
expr_index_index_call_unsupported
if_then_expr_index_target_var_index_int
if_else_expr_index_target_var_index_var
first_stmt_not_expr_unsupported
```

## Retire Candidate

```text
ExprIndexShapeSnapshotV1 for covered ProgramJSON Expr.expr Index rows
```

## Not Retired

- full Rust ASTNode projector;
- full index expression extractor or lowerer;
- index route selection or array get/set lowering;
- RecipeMatcher, route selection, MIR lowering, MIR mutation, or ID allocation;
- ProgramJSON full parser;
- HakoAdoption for a full owner;
- Source Selfhost.

## Acceptance

- retire fixture names only the covered expr-index rows;
- guard requires 3079 parity green;
- runtime Rust projector dependency removal remains `0`;
- Rust projector remains oracle-only;
- index route selection and array get lowering remain unclaimed.

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_expr_index_shape_retire_rust_astnode_projector_candidate_guard.sh
```

Decision fixture:

```text
docs/development/current/main/design/fixtures/rust-lifecycle/
  mirbuilder-programjson-expr-index-shape-retire-rust-astnode-projector-candidate-v0.json
```

## Next

```text
MIRBUILDER-PROGRAMJSON-CAPABILITY-BATCH-CONTINUATION
```
