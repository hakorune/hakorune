# 3015 - MIRBUILDER-PROGRAMJSON-EXPR-NEW-SHAPE-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001

Status: active

## Scope

Mark only the covered `ExprNewShapeSnapshotV1` ProgramJSON traversal rows as a
Rust ASTNode projector retire-candidate.

Covered rows:

```text
expr_new_no_args
expr_new_int_arg
expr_new_var_arg
expr_new_bool_arg
expr_new_compare_var_lt_int_arg
expr_new_compare_var_eq_int_arg
expr_new_call_arg_unsupported
if_then_expr_new_int_arg
if_else_expr_new_var_arg
```

## Requires

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_expr_new_shape_scan_parity_gate.sh
```

## Retire Candidate

```text
Rust ASTNode projector slice:
  ExprNewShapeSnapshotV1 for covered Expr.expr New ProgramJSON rows
```

## Not Retired

```text
full Rust ASTNode projector
full expression constructor extractor
RecipeMatcher
route selection
MIR lowering
MIR mutation
ID allocation
ProgramJSON full parser
HakoAdoption for a full owner
Source Selfhost
```

## Acceptance

- retire-candidate fixture names only the covered expr-new rows;
- guard requires the 3014 ProgramJSON expr-new parity gate to be green;
- runtime Rust projector dependency removal remains `0`;
- Rust projector remains oracle-only for this covered slice.
