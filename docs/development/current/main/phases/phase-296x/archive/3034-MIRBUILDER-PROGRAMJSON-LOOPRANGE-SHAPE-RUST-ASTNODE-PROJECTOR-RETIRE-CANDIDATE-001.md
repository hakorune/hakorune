# 3034 - MIRBUILDER-PROGRAMJSON-LOOPRANGE-SHAPE-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001

Status: landed

## Scope

After 3033 is parity-green, mark only the covered
`LoopRangeShapeSnapshotV1` ProgramJSON rows as a Rust ASTNode projector
retire-candidate.

This card does not retire the full Rust ASTNode projector and does not claim
LoopRange lowering or iterator/range runtime semantics.

## Requires

```text
bash tools/checks/rust_lifecycle_mirbuilder_programjson_looprange_shape_scan_parity_gate.sh
```

The 3033 gate must prove:

```text
capability=ProgramJsonLoopRangeShapeScanV1
output=LoopRangeShapeSnapshotV1
programjson_traversal_used=1
string_only_facade=0
rust_astnode_projector_retire_candidate=1
looprange_lowering=0
iterator_runtime_semantics=0
```

## Covered Rows

```text
top_looprange_int_to_int_empty_body
top_looprange_var_to_int_continue_body
top_looprange_int_to_var_return_body
top_looprange_var_to_var_break_body
if_then_looprange_int_to_int_empty_body
if_else_looprange_var_to_int_continue_body
top_looprange_float_bound_unsupported
top_looprange_nested_loop_unsupported
first_stmt_not_looprange_unsupported
```

## Retire Candidate

```text
LoopRangeShapeSnapshotV1
for covered ProgramJSON LoopRange statement rows
```

## Not Retired

```text
full Rust ASTNode projector
full LoopRange extractor
LoopRange lowering
iterator/range runtime semantics
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

- the 3033 parity gate is green;
- retire-candidate scope names only the covered rows above;
- Rust projector runtime dependency removal remains `0`;
- Rust projector oracle-only state remains documented;
- LoopRange lowering and iterator runtime semantics remain explicitly
  unclaimed.

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_looprange_shape_retire_rust_astnode_projector_candidate_guard.sh
```

Decision fixture:

```text
docs/development/current/main/design/fixtures/rust-lifecycle/
  mirbuilder-programjson-looprange-shape-retire-rust-astnode-projector-candidate-v0.json
```

Result:

```text
decision=RetireCandidateScoped
retire_candidate=LoopRangeShapeSnapshotV1
covered_rows=9
rust_projector_runtime_dependency_removed=0
rust_projector_oracle_only=1
full_astnode_projector_retired=0
looprange_lowering=0
iterator_runtime_semantics=0
```

## Next

```text
MIRBUILDER-PROGRAMJSON-CAPABILITY-BATCH-CONTINUATION
```
