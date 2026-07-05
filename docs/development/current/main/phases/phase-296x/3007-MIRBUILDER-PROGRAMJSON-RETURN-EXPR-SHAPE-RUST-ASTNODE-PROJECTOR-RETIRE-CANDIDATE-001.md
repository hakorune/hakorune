# 3007 - MIRBUILDER-PROGRAMJSON-RETURN-EXPR-SHAPE-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001

Status: landed

## Scope

Mark only the covered `ReturnExprShapeSnapshotV1` ProgramJSON traversal rows as
a Rust ASTNode projector retire-candidate.

Covered rows:

```text
top_return_int
top_return_var
top_return_bool_true
top_return_compare_var_lt_int
top_return_compare_var_eq_int
top_return_call_unsupported
if_then_return_int
if_else_return_var
```

## Requires

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_return_expr_shape_scan_parity_gate.sh
```

## Retire Candidate

```text
Rust ASTNode projector slice:
  ReturnExprShapeSnapshotV1 for covered Return.expr ProgramJSON rows
```

## Not Retired

```text
full Rust ASTNode projector
full return expression extractor
full loop_cond_continue_with_return facts extractor
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

- retire-candidate fixture names only the covered return-expr rows;
- guard requires the 3006 ProgramJSON return-expr parity gate to be green;
- runtime Rust projector dependency removal remains `0`;
- Rust projector remains oracle-only for this covered slice.

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_return_expr_shape_retire_rust_astnode_projector_candidate_guard.sh
```

Result:

```text
retire_candidate=ReturnExprShapeSnapshotV1
covered_rows=8
decision=RetireCandidateScoped
parity_gate=green
rust_projector_runtime_dependency_removed=0
rust_projector_oracle_only=1
full_astnode_projector_retired=0
source_selfhost_claim=0
```

## Next

```text
MIRBUILDER-PROGRAMJSON-LOCAL-BINDING-SHAPE-SCAN-CAPABILITY-001
```
