# 3054 - MIRBUILDER-PROGRAMJSON-TASK-SCOPE-SHAPE-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001

Status: landed

## Scope

After 3053 is parity-green, mark only the covered
`TaskScopeShapeSnapshotV1` ProgramJSON rows as a Rust ASTNode projector
retire-candidate.

This card does not retire the full Rust ASTNode projector and does not claim
task lowering or concurrency semantics.

## Requires

```text
bash tools/checks/rust_lifecycle_mirbuilder_programjson_task_scope_shape_scan_parity_gate.sh
```

The 3053 gate must prove:

```text
capability=ProgramJsonTaskScopeShapeScanV1
output=TaskScopeShapeSnapshotV1
programjson_traversal_used=1
string_only_facade=0
rust_astnode_projector_retire_candidate=1
task_lowering=0
concurrency_semantics=0
```

## Covered Rows

```text
top_task_empty
top_task_local
top_task_return
top_task_break
top_task_continue
top_task_loop
top_task_expr
if_then_task_local
if_else_task_empty
first_stmt_local_unsupported
```

## Retire Candidate

```text
TaskScopeShapeSnapshotV1
for covered ProgramJSON TaskScope statement body rows
```

## Not Retired

```text
full Rust ASTNode projector
full TaskScope extractor
task lowering
concurrency semantics
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

- the 3053 parity gate is green;
- retire-candidate scope names only the covered rows above;
- Rust projector runtime dependency removal remains `0`;
- Rust projector oracle-only state remains documented;
- task lowering and concurrency semantics remain explicitly unclaimed.

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_task_scope_shape_retire_rust_astnode_projector_candidate_guard.sh
```

Decision fixture:

```text
docs/development/current/main/design/fixtures/rust-lifecycle/
  mirbuilder-programjson-task-scope-shape-retire-rust-astnode-projector-candidate-v0.json
```

Result:

```text
decision=RetireCandidateScoped
retire_candidate=TaskScopeShapeSnapshotV1
covered_rows=10
rust_projector_runtime_dependency_removed=0
rust_projector_oracle_only=1
full_astnode_projector_retired=0
task_lowering=0
concurrency_semantics=0
```

## Next

```text
MIRBUILDER-PROGRAMJSON-CAPABILITY-BATCH-CONTINUATION
```
