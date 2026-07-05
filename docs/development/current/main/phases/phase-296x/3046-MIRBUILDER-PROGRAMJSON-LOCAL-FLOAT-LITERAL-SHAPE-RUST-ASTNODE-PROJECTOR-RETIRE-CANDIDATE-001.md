# 3046 - MIRBUILDER-PROGRAMJSON-LOCAL-FLOAT-LITERAL-SHAPE-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001

Status: landed

## Scope

After 3045 is parity-green, mark only the covered
`LocalFloatLiteralShapeSnapshotV1` ProgramJSON rows as a Rust ASTNode projector
retire-candidate.

This card does not retire the full Rust ASTNode projector and does not claim
float lowering or dynamic numeric typing.

## Requires

```text
bash tools/checks/rust_lifecycle_mirbuilder_programjson_local_float_literal_shape_scan_parity_gate.sh
```

The 3045 gate must prove:

```text
capability=ProgramJsonLocalFloatLiteralShapeScanV1
output=LocalFloatLiteralShapeSnapshotV1
programjson_traversal_used=1
string_only_facade=0
rust_astnode_projector_retire_candidate=1
float_lowering=0
dynamic_numeric_typing=0
```

## Covered Rows

```text
top_local_float_zero
top_local_float_one_point_five
top_local_float_negative
top_local_float_other
top_local_int_unsupported
top_local_str_unsupported
if_then_local_float
if_else_local_float
first_stmt_not_local_unsupported
```

## Retire Candidate

```text
LocalFloatLiteralShapeSnapshotV1
for covered ProgramJSON Local.expr Float literal rows
```

## Not Retired

```text
full Rust ASTNode projector
full Local extractor
float lowering
dynamic numeric typing
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

- the 3045 parity gate is green;
- retire-candidate scope names only the covered rows above;
- Rust projector runtime dependency removal remains `0`;
- Rust projector oracle-only state remains documented;
- float lowering and dynamic numeric typing remain explicitly unclaimed.

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_local_float_literal_shape_retire_rust_astnode_projector_candidate_guard.sh
```

Decision fixture:

```text
docs/development/current/main/design/fixtures/rust-lifecycle/
  mirbuilder-programjson-local-float-literal-shape-retire-rust-astnode-projector-candidate-v0.json
```

Result:

```text
decision=RetireCandidateScoped
retire_candidate=LocalFloatLiteralShapeSnapshotV1
covered_rows=9
rust_projector_runtime_dependency_removed=0
rust_projector_oracle_only=1
full_astnode_projector_retired=0
float_lowering=0
dynamic_numeric_typing=0
```

## Next

```text
MIRBUILDER-PROGRAMJSON-CAPABILITY-BATCH-CONTINUATION
```
