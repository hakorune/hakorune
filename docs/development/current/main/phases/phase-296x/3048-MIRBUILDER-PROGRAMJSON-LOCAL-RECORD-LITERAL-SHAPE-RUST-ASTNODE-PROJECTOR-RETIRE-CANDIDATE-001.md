# 3048 - MIRBUILDER-PROGRAMJSON-LOCAL-RECORD-LITERAL-SHAPE-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001

Status: landed

## Scope

After 3047 is parity-green, mark only the covered
`LocalRecordLiteralShapeSnapshotV1` ProgramJSON rows as a Rust ASTNode
projector retire-candidate.

This card does not retire the full Rust ASTNode projector and does not claim
record lowering or field-layout semantics.

## Requires

```text
bash tools/checks/rust_lifecycle_mirbuilder_programjson_local_record_literal_shape_scan_parity_gate.sh
```

The 3047 gate must prove:

```text
capability=ProgramJsonLocalRecordLiteralShapeScanV1
output=LocalRecordLiteralShapeSnapshotV1
programjson_traversal_used=1
string_only_facade=0
rust_astnode_projector_retire_candidate=1
record_lowering=0
field_layout_semantics=0
```

## Covered Rows

```text
top_local_point_no_fields
top_local_point_one_int_field
top_local_point_two_int_fields
top_local_config_str_bool_fields
top_local_record_three_fields_unsupported
top_local_int_unsupported
if_then_local_record_point_one_int_field
if_else_local_record_config_str_field
first_stmt_not_local_unsupported
```

## Retire Candidate

```text
LocalRecordLiteralShapeSnapshotV1
for covered ProgramJSON Local.expr RecordLiteral rows
```

## Not Retired

```text
full Rust ASTNode projector
full Local extractor
record lowering
field-layout semantics
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

- the 3047 parity gate is green;
- retire-candidate scope names only the covered rows above;
- Rust projector runtime dependency removal remains `0`;
- Rust projector oracle-only state remains documented;
- record lowering and field-layout semantics remain explicitly unclaimed.

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_local_record_literal_shape_retire_rust_astnode_projector_candidate_guard.sh
```

Decision fixture:

```text
docs/development/current/main/design/fixtures/rust-lifecycle/
  mirbuilder-programjson-local-record-literal-shape-retire-rust-astnode-projector-candidate-v0.json
```

Result:

```text
decision=RetireCandidateScoped
retire_candidate=LocalRecordLiteralShapeSnapshotV1
covered_rows=9
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
