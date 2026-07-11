# 3019 - MIRBUILDER-PROGRAMJSON-EXPR-RECORDFIELD-SHAPE-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001

Status: landed

## Scope

Mark only the covered `ExprRecordFieldShapeSnapshotV1` ProgramJSON traversal
rows as a Rust ASTNode projector retire-candidate.

Covered rows:

```text
expr_recordfield_recv_var
expr_recordfield_recv_new_no_args
expr_recordfield_recv_new_int_arg
expr_recordfield_recv_method_unsupported
expr_recordfield_recv_call_unsupported
if_then_expr_recordfield_recv_var
if_else_expr_recordfield_recv_new_no_args
plain_field_unsupported
first_stmt_not_expr_unsupported
```

## Requires

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_expr_recordfield_shape_scan_parity_gate.sh
```

## Retire Candidate

```text
Rust ASTNode projector slice:
  ExprRecordFieldShapeSnapshotV1 for covered Expr.expr RecordField ProgramJSON rows
```

## Not Retired

```text
full Rust ASTNode projector
full expression record field extractor
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

- retire-candidate fixture names only the covered expr-recordfield rows;
- guard requires the 3018 ProgramJSON expr-recordfield parity gate to be green;
- runtime Rust projector dependency removal remains `0`;
- Rust projector remains oracle-only for this covered slice.

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_expr_recordfield_shape_retire_rust_astnode_projector_candidate_guard.sh
```

Decision fixture:

```text
docs/development/current/main/design/fixtures/rust-lifecycle/
  mirbuilder-programjson-expr-recordfield-shape-retire-rust-astnode-projector-candidate-v0.json
```

## Next

```text
MIRBUILDER-PROGRAMJSON-EXPR-BINARY-SHAPE-SCAN-CAPABILITY-001
```
