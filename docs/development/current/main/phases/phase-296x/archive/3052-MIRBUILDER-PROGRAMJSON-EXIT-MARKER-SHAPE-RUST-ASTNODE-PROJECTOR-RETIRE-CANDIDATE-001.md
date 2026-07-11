# 3052 - MIRBUILDER-PROGRAMJSON-EXIT-MARKER-SHAPE-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001

Status: landed

## Scope

After 3051 is parity-green, mark only the covered
`ExitMarkerShapeSnapshotV1` ProgramJSON rows as a Rust ASTNode projector
retire-candidate.

This card does not retire the full Rust ASTNode projector and does not claim
CFG construction or exit lowering.

## Requires

```text
bash tools/checks/rust_lifecycle_mirbuilder_programjson_exit_marker_shape_scan_parity_gate.sh
```

The 3051 gate must prove:

```text
capability=ProgramJsonExitMarkerShapeScanV1
output=ExitMarkerShapeSnapshotV1
programjson_traversal_used=1
string_only_facade=0
rust_astnode_projector_retire_candidate=1
cfg_construction=0
exit_lowering=0
```

## Covered Rows

```text
top_break
top_continue
top_return_unsupported
loop_body_break
loop_body_continue
loop_body_local_unsupported
if_then_break
if_else_continue
if_then_continue_else_break
first_stmt_local_unsupported
```

## Retire Candidate

```text
ExitMarkerShapeSnapshotV1
for covered ProgramJSON Break/Continue marker rows
```

## Not Retired

```text
full Rust ASTNode projector
full exit marker extractor
CFG construction
exit lowering
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

- the 3051 parity gate is green;
- retire-candidate scope names only the covered rows above;
- Rust projector runtime dependency removal remains `0`;
- Rust projector oracle-only state remains documented;
- CFG construction and exit lowering remain explicitly unclaimed.

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_exit_marker_shape_retire_rust_astnode_projector_candidate_guard.sh
```

Decision fixture:

```text
docs/development/current/main/design/fixtures/rust-lifecycle/
  mirbuilder-programjson-exit-marker-shape-retire-rust-astnode-projector-candidate-v0.json
```

Result:

```text
decision=RetireCandidateScoped
retire_candidate=ExitMarkerShapeSnapshotV1
covered_rows=10
rust_projector_runtime_dependency_removed=0
rust_projector_oracle_only=1
full_astnode_projector_retired=0
cfg_construction=0
exit_lowering=0
```

## Next

```text
MIRBUILDER-PROGRAMJSON-CAPABILITY-BATCH-CONTINUATION
```
