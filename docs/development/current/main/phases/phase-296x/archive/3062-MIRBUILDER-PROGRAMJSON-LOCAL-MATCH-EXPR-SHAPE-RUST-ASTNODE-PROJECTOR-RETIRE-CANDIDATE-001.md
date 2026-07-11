# 3062 - MIRBUILDER-PROGRAMJSON-LOCAL-MATCH-EXPR-SHAPE-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001

Status: landed

## Scope

After 3061 is parity-green, mark only the covered
`LocalMatchExprShapeSnapshotV1` ProgramJSON rows as a Rust ASTNode projector
retire-candidate.

This card does not retire the full Rust ASTNode projector and does not claim
match lowering or branch execution semantics.

## Requires

```text
bash tools/checks/rust_lifecycle_mirbuilder_programjson_local_match_expr_shape_scan_parity_gate.sh
```

The 3061 gate must prove:

```text
capability=ProgramJsonLocalMatchExprShapeScanV1
output=LocalMatchExprShapeSnapshotV1
programjson_traversal_used=1
string_only_facade=0
rust_astnode_projector_retire_candidate=1
match_lowering=0
branch_execution_semantics=0
```

## Covered Rows

```text
top_var_one_int_arm_else_int
top_int_one_str_arm_else_str
top_one_arm_else_value
top_var_two_int_arms_else_int
top_var_bool_arms_else_bool
top_two_arms_else_value
top_three_arms_unsupported
if_then_var_one_int_arm_else_int
if_else_int_one_str_arm_else_str
first_stmt_return_unsupported
```

## Retire Candidate

```text
LocalMatchExprShapeSnapshotV1
for covered ProgramJSON Local.expr Match rows
```

## Not Retired

```text
full Rust ASTNode projector
full Local Match extractor
match lowering
branch execution semantics
RecipeMatcher
route selection
MIR lowering
MIR mutation
ID allocation
ProgramJSON full parser
HakoAdoption
Source Selfhost
new ABI
```

## Acceptance

- the 3061 parity gate is green;
- retire-candidate scope names only the covered rows above;
- Rust projector runtime dependency removal remains `0`;
- Rust projector oracle-only state remains documented;
- match lowering and branch execution semantics remain explicitly unclaimed.

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_local_match_expr_shape_retire_rust_astnode_projector_candidate_guard.sh
```

Decision fixture:

```text
docs/development/current/main/design/fixtures/rust-lifecycle/
  mirbuilder-programjson-local-match-expr-shape-retire-rust-astnode-projector-candidate-v0.json
```

Result:

```text
decision=RetireCandidateScoped
retire_candidate=LocalMatchExprShapeSnapshotV1
covered_rows=10
rust_projector_runtime_dependency_removed=0
rust_projector_oracle_only=1
full_astnode_projector_retired=0
match_lowering=0
branch_execution_semantics=0
```

## Next

```text
MIRBUILDER-PROGRAMJSON-CAPABILITY-BATCH-CONTINUATION
```
