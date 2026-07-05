# 3038 - MIRBUILDER-PROGRAMJSON-ASSIGN-SHAPE-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001

Status: landed

## Scope

After 3037 is parity-green, mark only the covered `AssignShapeSnapshotV1`
ProgramJSON rows as a Rust ASTNode projector retire-candidate.

This card does not retire the full Rust ASTNode projector and does not claim
assignment lowering or MIR mutation.

## Requires

```text
bash tools/checks/rust_lifecycle_mirbuilder_programjson_assign_shape_scan_parity_gate.sh
```

The 3037 gate must prove:

```text
capability=ProgramJsonAssignShapeScanV1
output=AssignShapeSnapshotV1
programjson_traversal_used=1
string_only_facade=0
rust_astnode_projector_retire_candidate=1
mir_mutation=0
```

## Covered Rows

```text
top_assign_var_int
top_assign_var_var
top_assign_var_bool_true
top_assign_var_compare_lt
top_assign_var_binary_add
top_assign_target_field_unsupported
top_assign_call_unsupported
if_then_assign_var_int
if_else_assign_var_var
first_stmt_not_assign_unsupported
```

## Retire Candidate

```text
AssignShapeSnapshotV1
for covered ProgramJSON Assign statement rows
```

## Not Retired

```text
full Rust ASTNode projector
full Assign extractor
assignment lowering
MIR mutation
RecipeMatcher
route selection
MIR lowering
ID allocation
ProgramJSON full parser
HakoAdoption
Source Selfhost
new ABI
```

## Acceptance

- the 3037 parity gate is green;
- retire-candidate scope names only the covered rows above;
- Rust projector runtime dependency removal remains `0`;
- Rust projector oracle-only state remains documented;
- assignment lowering and MIR mutation remain explicitly unclaimed.

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_assign_shape_retire_rust_astnode_projector_candidate_guard.sh
```

Decision fixture:

```text
docs/development/current/main/design/fixtures/rust-lifecycle/
  mirbuilder-programjson-assign-shape-retire-rust-astnode-projector-candidate-v0.json
```

Result:

```text
decision=RetireCandidateScoped
retire_candidate=AssignShapeSnapshotV1
covered_rows=10
rust_projector_runtime_dependency_removed=0
rust_projector_oracle_only=1
full_astnode_projector_retired=0
mir_mutation=0
assignment_lowering=0
```

## Next

```text
MIRBUILDER-PROGRAMJSON-CAPABILITY-BATCH-CONTINUATION
```
