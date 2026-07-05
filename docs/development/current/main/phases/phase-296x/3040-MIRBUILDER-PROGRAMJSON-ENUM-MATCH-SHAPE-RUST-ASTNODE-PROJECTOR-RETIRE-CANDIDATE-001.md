# 3040 - MIRBUILDER-PROGRAMJSON-ENUM-MATCH-SHAPE-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001

Status: landed

## Scope

After 3039 is parity-green, mark only the covered
`EnumMatchShapeSnapshotV1` ProgramJSON rows as a Rust ASTNode projector
retire-candidate.

This card does not retire the full Rust ASTNode projector and does not claim
match lowering or enum runtime semantics.

## Requires

```text
bash tools/checks/rust_lifecycle_mirbuilder_programjson_enum_match_shape_scan_parity_gate.sh
```

The 3039 gate must prove:

```text
capability=ProgramJsonEnumMatchShapeScanV1
output=EnumMatchShapeSnapshotV1
programjson_traversal_used=1
string_only_facade=0
rust_astnode_projector_retire_candidate=1
match_lowering_semantics=0
enum_runtime_semantics=0
```

## Covered Rows

```text
top_option_some_bool_true_none_bool_false
top_option_some_bind_var_none_null
top_result_enum_unsupported
top_option_scrutinee_call_unsupported
top_option_arm_order_unsupported
top_option_three_arms_unsupported
if_then_option_some_bool_true_none_bool_false
if_else_option_some_bind_var_none_null
first_stmt_not_enum_match_unsupported
```

## Retire Candidate

```text
EnumMatchShapeSnapshotV1
for covered ProgramJSON EnumMatch rows
```

## Not Retired

```text
full Rust ASTNode projector
full EnumMatch extractor
match lowering
enum runtime semantics
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

- the 3039 parity gate is green;
- retire-candidate scope names only the covered rows above;
- Rust projector runtime dependency removal remains `0`;
- Rust projector oracle-only state remains documented;
- match lowering and enum runtime semantics remain explicitly unclaimed.

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_enum_match_shape_retire_rust_astnode_projector_candidate_guard.sh
```

Decision fixture:

```text
docs/development/current/main/design/fixtures/rust-lifecycle/
  mirbuilder-programjson-enum-match-shape-retire-rust-astnode-projector-candidate-v0.json
```

Result:

```text
decision=RetireCandidateScoped
retire_candidate=EnumMatchShapeSnapshotV1
covered_rows=9
rust_projector_runtime_dependency_removed=0
rust_projector_oracle_only=1
full_astnode_projector_retired=0
match_lowering_semantics=0
enum_runtime_semantics=0
```

## Next

```text
MIRBUILDER-PROGRAMJSON-CAPABILITY-BATCH-CONTINUATION
```
