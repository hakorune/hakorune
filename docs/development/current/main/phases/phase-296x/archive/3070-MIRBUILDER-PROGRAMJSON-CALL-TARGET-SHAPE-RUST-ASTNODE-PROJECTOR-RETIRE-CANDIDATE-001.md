# 3070 - MIRBUILDER-PROGRAMJSON-CALL-TARGET-SHAPE-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001

Status: landed

## Scope

After 3069 is parity-green, mark only the covered
`CallTargetShapeSnapshotV1` ProgramJSON rows as a Rust ASTNode projector
retire-candidate.

This card does not retire the full Rust ASTNode projector and does not claim
call resolution or dispatch selection.

## Requires

```text
bash tools/checks/rust_lifecycle_mirbuilder_programjson_call_target_shape_scan_parity_gate.sh
```

The 3069 gate must prove:

```text
capability=ProgramJsonCallTargetShapeScanV1
output=CallTargetShapeSnapshotV1
programjson_traversal_used=1
string_only_facade=0
rust_astnode_projector_retire_candidate=1
call_resolution=0
dispatch_selection=0
```

## Covered Rows

```text
expr_env_console_log_one_arg
local_simple_no_args
local_simple_one_arg
return_simple_two_args
local_dotted_static_no_args
local_dotted_static_two_args
local_to_i64_one_arg
local_int_to_str_one_arg
if_then_simple_one_arg
if_else_dotted_static_one_arg
```

## Retire Candidate

```text
CallTargetShapeSnapshotV1
for covered ProgramJSON Call.name/arity rows
```

## Not Retired

```text
full Rust ASTNode projector
full Call target extractor
call resolution
dispatch selection
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

- the 3069 parity gate is green;
- retire-candidate scope names only the covered rows above;
- Rust projector runtime dependency removal remains `0`;
- Rust projector oracle-only state remains documented;
- call resolution and dispatch selection remain explicitly unclaimed.

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_call_target_shape_retire_rust_astnode_projector_candidate_guard.sh
```

Decision fixture:

```text
docs/development/current/main/design/fixtures/rust-lifecycle/
  mirbuilder-programjson-call-target-shape-retire-rust-astnode-projector-candidate-v0.json
```

Result:

```text
decision=RetireCandidateScoped
retire_candidate=CallTargetShapeSnapshotV1
covered_rows=10
rust_projector_runtime_dependency_removed=0
rust_projector_oracle_only=1
full_astnode_projector_retired=0
call_resolution=0
dispatch_selection=0
```

## Next

```text
MIRBUILDER-PROGRAMJSON-CAPABILITY-BATCH-CONTINUATION
```
