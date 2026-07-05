# 2995 - MIRBUILDER-PROGRAMJSON-LOOP-BODY-CONTROL-FLOW-SCAN-CAPABILITY-001

Status: landed

## Scope

Implement the first ProgramJSON traversal capability batch:

```text
ProgramJsonLoopBodyControlFlowScanV1
```

This owner consumes Program(JSON v0), structurally locates the first top-level
`Loop.body`, scans the covered first/second statement shapes, and emits:

```text
LoopBodyControlFlowSnapshotV1
```

Covered rows:

```text
empty_body
return_only
continue_only
break_present
continue_then_return
if_then_continue_else_null_then_return
nested_loop
if_hidden_nested_loop
if_then_continue_no_return
second_stmt_not_return
```

## Evidence

Implementation:

```text
lang/src/compiler/mirbuilder/program_json_loop_body_control_flow_scan.hako
```

Fixture:

```text
docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-loop-body-control-flow-scan-parity-v0.json
```

Gate:

```text
tools/checks/rust_lifecycle_mirbuilder_programjson_loop_body_control_flow_scan_parity_gate.sh
```

Green output:

```text
parity_rows=10
covered_rows=10
execution_backend=aot
aot_execution_status=green
token_snapshot_parity=green
programjson_traversal_used=1
string_only_facade=0
rust_astnode_projector_retire_candidate=1
```

## Implementation Notes

This slice intentionally avoids comparing returned dynamic string values for
node type equality and avoids `StringHelpers.int_to_str` for dynamic count
values. The `.hako` AOT path exposed that both are unsafe for this capability.

Instead, node recognition reads the ProgramJSON `type` field through the scanner
and checks the first quoted value character. Count output is converted through a
bounded `0/1/2` token helper.

## Next

Next card:

```text
MIRBUILDER-PROGRAMJSON-LOOP-BODY-CONTROL-FLOW-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001
```

It may mark only the covered `LoopBodyControlFlowSnapshotV1` rows as a Rust
ASTNode projector retire-candidate.

## Non-Claims

- `source_selfhost_claim = 0`
- `hako_adopted_decision = 0`
- `rust_astnode_projector_retired = 0`
- `rust_astnode_projector_fully_retired = 0`
- `programjson_full_parser_claim = 0`
- `programjson_all_shapes_supported = 0`
- `recipe_matching_migrated = 0`
- `route_selection_migration = 0`
- `backend_lowering_migration = 0`
- `mir_mutation_migration = 0`
- `id_allocation_migration = 0`
- `new_backend_route = 0`
- `new_abi = 0`
