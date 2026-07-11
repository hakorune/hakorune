# 3105 - MIRBUILDER-PROGRAMJSON-TRYCATCH-SHAPE-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001

Status: landed

## Scope

Mark only the covered `TryCatchShapeSnapshotV1` ProgramJSON traversal rows as a
scoped Rust ASTNode projector retire-candidate after 3104 parity is green.

This is a future runtime route-switch candidate, not deletion of Rust
bootstrap/oracle code. It does not add exception runtime semantics, catch
matching, or cleanup execution semantics.

## Requires

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_trycatch_shape_scan_parity_gate.sh
```

## Retire Candidate Scope

```text
TryCatchShapeSnapshotV1
for covered ProgramJSON TryCatch rows
```

Covered rows:

```text
try_throw_no_catch_no_cleanup
try_return_one_catch_no_cleanup
try_expr_one_catch_cleanup_expr
try_empty_many_catches_no_cleanup
try_return_no_catch_cleanup_return
nested_trycatch_unsupported
cleanup_scalar_unsupported
first_stmt_local_unsupported
```

## Not Retired

```text
full Rust ASTNode projector
exception runtime semantics
catch matching
cleanup execution semantics
RecipeMatcher
route selection
MIR lowering
MIR mutation
ID allocation
ProgramJSON full parser
HakoAdoption for a full owner
Source Selfhost
```

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_trycatch_shape_retire_rust_astnode_projector_candidate_guard.sh
```

Guard result:

```text
decision=RetireCandidateScoped
parity_gate=green
covered_rows=8
exception_runtime_semantics=0
catch_matching=0
cleanup_execution_semantics=0
full_astnode_projector_retired=0
```

## Next

```text
MIRBUILDER-PROGRAMJSON-LAYER4-STRUCTURED-PLAN-RECIPE-DTO-PILOT-SELECTION
```
