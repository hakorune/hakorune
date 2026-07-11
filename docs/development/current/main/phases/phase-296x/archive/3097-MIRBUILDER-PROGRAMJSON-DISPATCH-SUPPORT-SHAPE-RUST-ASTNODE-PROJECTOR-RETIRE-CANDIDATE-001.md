# 3097 - MIRBUILDER-PROGRAMJSON-DISPATCH-SUPPORT-SHAPE-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001

Status: landed

## Scope

Mark only the covered `DispatchSupportShapeSnapshotV1` ProgramJSON traversal
rows as a scoped Rust ASTNode projector retire-candidate after 3096 parity is
green.

This does not retire the full Rust ASTNode projector and does not add support
for unsupported ProgramJSON statements.

## Requires

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_dispatch_support_shape_scan_parity_gate.sh
```

## Retire Candidate Scope

```text
DispatchSupportShapeSnapshotV1
for covered ProgramJSON statement dispatch-support rows
```

Covered rows:

```text
top_local_dispatchable
top_loop_dispatchable
top_try_dispatchable_cleanup
top_break_unsupported_exit
top_context_scope_unsupported
top_extern_unsupported
top_function_declaration_kind_only_unsupported
top_looprange_unsupported_other
if_then_return_else_break
if_then_task_scope_else_local
```

## Not Retired

```text
full Rust ASTNode projector
full ProgramJSON phase-state consumer
unsupported statement dispatch
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
bash tools/checks/rust_lifecycle_mirbuilder_programjson_dispatch_support_shape_retire_rust_astnode_projector_candidate_guard.sh
```

Guard result:

```text
decision=RetireCandidateScoped
parity_gate=green
covered_rows=10
dispatch_support_added=0
unsupported_stmt_resolved=0
full_astnode_projector_retired=0
```

## Next

```text
MIRBUILDER-PROGRAMJSON-CAPABILITY-BATCH-CONTINUATION
```
