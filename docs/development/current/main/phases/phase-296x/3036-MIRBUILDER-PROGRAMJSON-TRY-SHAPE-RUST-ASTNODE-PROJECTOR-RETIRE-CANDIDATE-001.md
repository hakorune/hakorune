# 3036 - MIRBUILDER-PROGRAMJSON-TRY-SHAPE-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001

Status: landed

## Scope

After 3035 is parity-green, mark only the covered `TryShapeSnapshotV1`
ProgramJSON rows as a Rust ASTNode projector retire-candidate.

This card does not retire the full Rust ASTNode projector and does not claim
exception runtime semantics, catch matching, or finally execution semantics.

## Requires

```text
bash tools/checks/rust_lifecycle_mirbuilder_programjson_try_shape_scan_parity_gate.sh
```

The 3035 gate must prove:

```text
capability=ProgramJsonTryShapeScanV1
output=TryShapeSnapshotV1
programjson_traversal_used=1
string_only_facade=0
rust_astnode_projector_retire_candidate=1
exception_runtime_semantics=0
catch_matching=0
finally_execution_semantics=0
```

## Covered Rows

```text
top_try_throw_no_catch_no_finally
top_try_return_one_catch_no_finally
top_try_expr_one_catch_finally_expr
top_try_empty_many_catches_no_finally
top_try_return_no_catch_finally_return
if_then_try_throw_no_catch_no_finally
if_else_try_return_one_catch_no_finally
top_try_nested_try_unsupported
first_stmt_not_try_unsupported
```

## Retire Candidate

```text
TryShapeSnapshotV1
for covered ProgramJSON Try statement rows
```

## Not Retired

```text
full Rust ASTNode projector
full Try extractor
exception runtime semantics
catch matching
finally execution semantics
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

- the 3035 parity gate is green;
- retire-candidate scope names only the covered rows above;
- Rust projector runtime dependency removal remains `0`;
- Rust projector oracle-only state remains documented;
- exception runtime semantics, catch matching, and finally execution semantics
  remain explicitly unclaimed.

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_try_shape_retire_rust_astnode_projector_candidate_guard.sh
```

Decision fixture:

```text
docs/development/current/main/design/fixtures/rust-lifecycle/
  mirbuilder-programjson-try-shape-retire-rust-astnode-projector-candidate-v0.json
```

Result:

```text
decision=RetireCandidateScoped
retire_candidate=TryShapeSnapshotV1
covered_rows=9
rust_projector_runtime_dependency_removed=0
rust_projector_oracle_only=1
full_astnode_projector_retired=0
exception_runtime_semantics=0
catch_matching=0
finally_execution_semantics=0
```

## Next

```text
MIRBUILDER-PROGRAMJSON-CAPABILITY-BATCH-CONTINUATION
```
