# 3058 - MIRBUILDER-PROGRAMJSON-LOCAL-RECORD-UPDATE-SHAPE-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001

Status: landed

## Scope

After 3057 is parity-green, mark only the covered
`LocalRecordUpdateShapeSnapshotV1` ProgramJSON rows as a Rust ASTNode projector
retire-candidate.

This card does not retire the full Rust ASTNode projector and does not claim
record lowering or field layout semantics.

## Requires

```text
bash tools/checks/rust_lifecycle_mirbuilder_programjson_local_record_update_shape_scan_parity_gate.sh
```

The 3057 gate must prove:

```text
capability=ProgramJsonLocalRecordUpdateShapeScanV1
output=LocalRecordUpdateShapeSnapshotV1
programjson_traversal_used=1
string_only_facade=0
rust_astnode_projector_retire_candidate=1
record_lowering=0
field_layout_semantics=0
```

## Covered Rows

```text
top_point_no_updates
top_point_one_int_update
top_config_one_str_update
top_other_one_bool_update
top_other_one_var_update
top_point_two_int_updates
top_config_str_bool_updates
if_then_point_one_int_update
if_else_point_no_updates
first_stmt_return_unsupported
```

## Retire Candidate

```text
LocalRecordUpdateShapeSnapshotV1
for covered ProgramJSON Local.expr RecordUpdate rows
```

## Not Retired

```text
full Rust ASTNode projector
full Local RecordUpdate extractor
record lowering
field layout semantics
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

- the 3057 parity gate is green;
- retire-candidate scope names only the covered rows above;
- Rust projector runtime dependency removal remains `0`;
- Rust projector oracle-only state remains documented;
- record lowering and field layout semantics remain explicitly unclaimed.

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_local_record_update_shape_retire_rust_astnode_projector_candidate_guard.sh
```

Decision fixture:

```text
docs/development/current/main/design/fixtures/rust-lifecycle/
  mirbuilder-programjson-local-record-update-shape-retire-rust-astnode-projector-candidate-v0.json
```

Result:

```text
decision=RetireCandidateScoped
retire_candidate=LocalRecordUpdateShapeSnapshotV1
covered_rows=10
rust_projector_runtime_dependency_removed=0
rust_projector_oracle_only=1
full_astnode_projector_retired=0
record_lowering=0
field_layout_semantics=0
```

## Next

```text
MIRBUILDER-PROGRAMJSON-CAPABILITY-BATCH-CONTINUATION
```
