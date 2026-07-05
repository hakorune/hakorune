# 3044 - MIRBUILDER-PROGRAMJSON-LOCAL-NULL-LITERAL-SHAPE-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001

Status: landed

## Scope

After 3043 is parity-green, mark only the covered
`LocalNullLiteralShapeSnapshotV1` ProgramJSON rows as a Rust ASTNode projector
retire-candidate.

This card does not retire the full Rust ASTNode projector and does not claim
null lowering or option semantics.

## Requires

```text
bash tools/checks/rust_lifecycle_mirbuilder_programjson_local_null_literal_shape_scan_parity_gate.sh
```

The 3043 gate must prove:

```text
capability=ProgramJsonLocalNullLiteralShapeScanV1
output=LocalNullLiteralShapeSnapshotV1
programjson_traversal_used=1
string_only_facade=0
rust_astnode_projector_retire_candidate=1
null_lowering=0
option_semantics=0
```

## Covered Rows

```text
top_local_null_explicit
top_local_null_declared_box
top_local_null_no_initializer_projection
top_local_int_unsupported
top_local_str_unsupported
if_then_local_null
if_else_local_null
if_then_return_unsupported
first_stmt_not_local_unsupported
```

## Retire Candidate

```text
LocalNullLiteralShapeSnapshotV1
for covered ProgramJSON Local.expr Null literal rows
```

## Not Retired

```text
full Rust ASTNode projector
full Local extractor
null lowering
option semantics
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

- the 3043 parity gate is green;
- retire-candidate scope names only the covered rows above;
- Rust projector runtime dependency removal remains `0`;
- Rust projector oracle-only state remains documented;
- null lowering and option semantics remain explicitly unclaimed.

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_local_null_literal_shape_retire_rust_astnode_projector_candidate_guard.sh
```

Decision fixture:

```text
docs/development/current/main/design/fixtures/rust-lifecycle/
  mirbuilder-programjson-local-null-literal-shape-retire-rust-astnode-projector-candidate-v0.json
```

Result:

```text
decision=RetireCandidateScoped
retire_candidate=LocalNullLiteralShapeSnapshotV1
covered_rows=9
rust_projector_runtime_dependency_removed=0
rust_projector_oracle_only=1
full_astnode_projector_retired=0
null_lowering=0
option_semantics=0
```

## Next

```text
MIRBUILDER-PROGRAMJSON-CAPABILITY-BATCH-CONTINUATION
```
