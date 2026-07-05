# 3094 - MIRBUILDER-PROGRAMJSON-EXPR-ME-SHAPE-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001

Status: landed

## Scope

After 3093 is green, mark only the covered `ExprMeShapeSnapshotV1` ProgramJSON
traversal rows as a Rust ASTNode projector retire-candidate.

## Requires

```text
bash tools/checks/rust_lifecycle_mirbuilder_programjson_expr_me_shape_scan_parity_gate.sh
```

## Covered Rows

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

## Retire Candidate

```text
ExprMeShapeSnapshotV1 for covered ProgramJSON Expr.expr Me rows
```

## Not Retired

- full Rust ASTNode projector;
- full Me extractor or lowerer;
- receiver-origin resolution or `me` lowering;
- RecipeMatcher, route selection, MIR lowering, MIR mutation, or ID allocation;
- ProgramJSON full parser;
- HakoAdoption for a full owner;
- Source Selfhost.

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_expr_me_shape_retire_rust_astnode_projector_candidate_guard.sh
```

Decision fixture:

```text
docs/development/current/main/design/fixtures/rust-lifecycle/
  mirbuilder-programjson-expr-me-shape-retire-rust-astnode-projector-candidate-v0.json
```

## Next

```text
MIRBUILDER-PROGRAMJSON-CAPABILITY-BATCH-CONTINUATION
```
