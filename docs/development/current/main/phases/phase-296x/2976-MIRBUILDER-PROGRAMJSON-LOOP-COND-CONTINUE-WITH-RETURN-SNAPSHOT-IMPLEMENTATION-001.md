---
Status: Landed
Date: 2026-07-05
Scope: First ProgramJSON-to-token-snapshot owner implementation for MirBuilder migration.
---

# MIRBUILDER-PROGRAMJSON-LOOP-COND-CONTINUE-WITH-RETURN-SNAPSHOT-IMPLEMENTATION-001

## Result

Implemented the first read-only ProgramJSON traversal owner:

```text
owner=ProgramJsonLoopCondContinueWithReturnSnapshotV1
implementation=lang/src/compiler/mirbuilder/program_json_loop_cond_continue_with_return_snapshot.hako
fixture=docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-loop-cond-continue-with-return-snapshot-implementation-v0.json
gate=tools/checks/rust_lifecycle_mirbuilder_programjson_loop_cond_continue_with_return_snapshot_implementation_gate.sh
```

The owner traverses this one Program(JSON v0) shape:

```text
Program.body[0]=Loop(cond=Compare, body=[
  If(cond=Compare, then=[Continue], else=null),
  Return(Int)
])
```

and emits the canonical snapshot:

```text
snapshot_kind=LoopCondContinueWithReturnProgramJsonSnapshotV1
loop_condition_valid=1
continue_count=1
break_count=0
return_count=1
has_nested_loop=0
continue_if_count=1
then_tail_continue=1
else_is_null=1
hetero_return_if_count=0
unsupported_node_count=0
```

## Gate Evidence

```text
tools/checks/rust_lifecycle_mirbuilder_programjson_loop_cond_continue_with_return_snapshot_implementation_gate.sh
```

Result:

```text
mir_verify_status=green
implementation_rows=8
execution_backend=not_run
aot_execution_status=blocked_by_existing_program_json_v0_scanner_lowering
parity_status=not_claimed
```

The gate verifies:

```text
1. ProgramJsonV0ScannerBox MIR verification is green.
2. ProgramJsonLoopCondContinueWithReturnSnapshotBox MIR verification is green.
3. A generated app calling the new owner over all fixture rows verifies as MIR.
```

It does not execute the generated app. Execution is blocked before this owner
can be proven at runtime because the existing scanner helper is not AOT
lowerable:

```text
callee_symbol=ProgramJsonV0ScannerBox.seek_obj_field_value_start/3
reason=module_generic_prepass_failed
owner_hint=backend_lowering
```

## Boundaries

This card does not claim:

```text
source_selfhost_claim=0
hako_adopted_decision=0
rust_astnode_projector_retired=0
programjson_full_parser_claim=0
parity_status=not_claimed
recipe_matching_migrated=0
route_selection_migration=0
backend_lowering_migration=0
mir_mutation_migration=0
id_allocation_migration=0
```

## Next

Before the parity card can be honest, either the ProgramJSON scanner execution
route must be made gateable or the parity contract must be explicitly scoped to
a non-AOT runner.

```text
MIRBUILDER-PROGRAMJSON-V0-SCANNER-AOT-LOWERING-BLOCKER-INVENTORY-001
```
