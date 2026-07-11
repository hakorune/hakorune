# 3011 - MIRBUILDER-PROGRAMJSON-EXPR-CALL-SHAPE-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001

Status: landed

## Scope

Mark only the covered `ExprCallShapeSnapshotV1` ProgramJSON traversal rows as a
Rust ASTNode projector retire-candidate.

Covered rows:

```text
expr_call_no_args
expr_call_int_arg
expr_call_var_arg
expr_call_bool_arg
expr_call_compare_var_lt_int_arg
expr_call_compare_var_eq_int_arg
expr_call_call_arg_unsupported
if_then_expr_call_int_arg
if_else_expr_call_var_arg
```

## Requires

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_expr_call_shape_scan_parity_gate.sh
```

## Retire Candidate

```text
Rust ASTNode projector slice:
  ExprCallShapeSnapshotV1 for covered Expr.expr Call ProgramJSON rows
```

## Not Retired

```text
full Rust ASTNode projector
full expression call extractor
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

- retire-candidate fixture names only the covered expr-call rows;
- guard requires the 3010 ProgramJSON expr-call parity gate to be green;
- runtime Rust projector dependency removal remains `0`;
- Rust projector remains oracle-only for this covered slice.

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_expr_call_shape_retire_rust_astnode_projector_candidate_guard.sh
```

Decision fixture:

```text
docs/development/current/main/design/fixtures/rust-lifecycle/
  mirbuilder-programjson-expr-call-shape-retire-rust-astnode-projector-candidate-v0.json
```

## Next

```text
MIRBUILDER-PROGRAMJSON-EXPR-METHOD-SHAPE-SCAN-CAPABILITY-001
```
