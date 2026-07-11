# 3050 - MIRBUILDER-PROGRAMJSON-LOCAL-MAP-LITERAL-SHAPE-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001

Status: landed

## Scope

After 3049 is parity-green, mark only the covered
`LocalMapLiteralShapeSnapshotV1` ProgramJSON rows as a Rust ASTNode projector
retire-candidate.

This card does not retire the full Rust ASTNode projector and does not claim
MapBox lowering or map allocation semantics.

## Requires

```text
bash tools/checks/rust_lifecycle_mirbuilder_programjson_local_map_literal_shape_scan_parity_gate.sh
```

The 3049 gate must prove:

```text
capability=ProgramJsonLocalMapLiteralShapeScanV1
output=LocalMapLiteralShapeSnapshotV1
programjson_traversal_used=1
string_only_facade=0
rust_astnode_projector_retire_candidate=1
map_lowering=0
map_allocation_semantics=0
```

## Covered Rows

```text
top_local_map_empty
top_local_map_one_int
top_local_map_one_str
top_local_map_one_bool
top_local_map_two_int_str
top_local_map_three_entries_unsupported
top_local_int_unsupported
if_then_local_map_one_int
if_else_local_map_empty
first_stmt_not_local_unsupported
```

## Retire Candidate

```text
LocalMapLiteralShapeSnapshotV1
for covered ProgramJSON Local.expr Map literal rows
```

## Not Retired

```text
full Rust ASTNode projector
full Local extractor
MapBox lowering
map allocation semantics
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

- the 3049 parity gate is green;
- retire-candidate scope names only the covered rows above;
- Rust projector runtime dependency removal remains `0`;
- Rust projector oracle-only state remains documented;
- MapBox lowering and map allocation semantics remain explicitly unclaimed.

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_local_map_literal_shape_retire_rust_astnode_projector_candidate_guard.sh
```

Decision fixture:

```text
docs/development/current/main/design/fixtures/rust-lifecycle/
  mirbuilder-programjson-local-map-literal-shape-retire-rust-astnode-projector-candidate-v0.json
```

Result:

```text
decision=RetireCandidateScoped
retire_candidate=LocalMapLiteralShapeSnapshotV1
covered_rows=10
rust_projector_runtime_dependency_removed=0
rust_projector_oracle_only=1
full_astnode_projector_retired=0
map_lowering=0
map_allocation_semantics=0
```

## Next

```text
MIRBUILDER-PROGRAMJSON-CAPABILITY-BATCH-CONTINUATION
```
