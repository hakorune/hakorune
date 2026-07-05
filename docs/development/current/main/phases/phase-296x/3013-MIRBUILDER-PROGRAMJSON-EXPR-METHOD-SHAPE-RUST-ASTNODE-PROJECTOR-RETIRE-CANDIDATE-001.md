# 3013 - MIRBUILDER-PROGRAMJSON-EXPR-METHOD-SHAPE-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001

Status: active

## Scope

Mark only the covered `ExprMethodShapeSnapshotV1` ProgramJSON traversal rows as
a Rust ASTNode projector retire-candidate.

Covered rows:

```text
expr_method_no_args
expr_method_int_arg
expr_method_var_arg
expr_method_bool_arg
expr_method_compare_var_lt_int_arg
expr_method_compare_var_eq_int_arg
expr_method_call_arg_unsupported
if_then_expr_method_int_arg
if_else_expr_method_var_arg
```

## Requires

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_expr_method_shape_scan_parity_gate.sh
```

## Retire Candidate

```text
Rust ASTNode projector slice:
  ExprMethodShapeSnapshotV1 for covered Expr.expr Method ProgramJSON rows
```

## Not Retired

```text
full Rust ASTNode projector
full expression method extractor
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

- retire-candidate fixture names only the covered expr-method rows;
- guard requires the 3012 ProgramJSON expr-method parity gate to be green;
- runtime Rust projector dependency removal remains `0`;
- Rust projector remains oracle-only for this covered slice.
