# 3096 - MIRBUILDER-PROGRAMJSON-DISPATCH-SUPPORT-SHAPE-SCAN-CAPABILITY-001

Status: landed

## Scope

Add real `.hako` ProgramJSON traversal for covered statement dispatch-support
shapes. The owner reads ProgramJSON v0 statement nodes and classifies whether
the current phase-state consumer has a dispatch path for each covered node.

This observes dispatch support only. It does not add support for unsupported
statements.

## Implemented Owner

```text
ProgramJsonDispatchSupportShapeScanV1
```

Output:

```text
DispatchSupportShapeSnapshotV1
```

Covered shape kinds:

```text
DispatchableNonControl
DispatchableControl
DispatchableTryCleanup
UnsupportedExitMarker
UnsupportedScope
UnsupportedExternal
UnsupportedDeclaration
UnsupportedOther
Missing
```

## Parity Rows

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

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_dispatch_support_shape_scan_parity_gate.sh
```

Gate result:

```text
aot_execution_status=green
token_snapshot_parity=green
programjson_traversal_used=1
string_only_facade=0
rust_astnode_projector_retire_candidate=1
dispatch_support_added=0
unsupported_stmt_resolved=0
```

## Explicit Non-Claims

- dispatch support added;
- unsupported statement resolution;
- full Rust ASTNode projector retirement;
- HakoAdoption for a full owner;
- ProgramJSON full parser;
- RecipeMatcher, route selection, MIR lowering, MIR mutation, or ID allocation;
- Source Selfhost;
- new backend route or ABI.

## Next

```text
MIRBUILDER-PROGRAMJSON-DISPATCH-SUPPORT-SHAPE-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001
```
