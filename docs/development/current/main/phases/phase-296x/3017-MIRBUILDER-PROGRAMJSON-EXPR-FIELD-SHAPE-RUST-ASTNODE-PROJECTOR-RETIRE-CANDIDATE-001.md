# 3017 - MIRBUILDER-PROGRAMJSON-EXPR-FIELD-SHAPE-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001

Status: active

## Scope

Mark only the covered `ExprFieldShapeSnapshotV1` ProgramJSON traversal rows as
a Rust ASTNode projector retire-candidate.

Covered rows:

```text
expr_field_recv_var
expr_field_recv_new_no_args
expr_field_recv_new_int_arg
expr_field_recv_method_unsupported
expr_field_recv_call_unsupported
if_then_expr_field_recv_var
if_else_expr_field_recv_new_no_args
record_field_unsupported
first_stmt_not_expr_unsupported
```

## Requires

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_expr_field_shape_scan_parity_gate.sh
```

## Retire Candidate

```text
Rust ASTNode projector slice:
  ExprFieldShapeSnapshotV1 for covered Expr.expr Field ProgramJSON rows
```

## Not Retired

```text
full Rust ASTNode projector
full expression field extractor
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

- retire-candidate fixture names only the covered expr-field rows;
- guard requires the 3016 ProgramJSON expr-field parity gate to be green;
- runtime Rust projector dependency removal remains `0`;
- Rust projector remains oracle-only for this covered slice.
