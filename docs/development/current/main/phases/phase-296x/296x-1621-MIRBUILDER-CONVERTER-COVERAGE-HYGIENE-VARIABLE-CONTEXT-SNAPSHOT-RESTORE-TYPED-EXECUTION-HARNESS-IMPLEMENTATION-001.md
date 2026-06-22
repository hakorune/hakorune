# 296x-1621 MIRBUILDER-CONVERTER-COVERAGE-HYGIENE-VARIABLE-CONTEXT-SNAPSHOT-RESTORE-TYPED-EXECUTION-HARNESS-IMPLEMENTATION-001

Status: Active
Date: 2026-06-22

## Purpose

Implement the typed Main harness for the selected VariableContext
snapshot/restore slice. The family already has live snapshot/restore behavior
and alias-proof native coverage; this slice only moves the derived artifact
execution harness from raw `main_lines` to typed operations.

## Scope

```text
BoxCount: one implementation contract
owner: MirBuilder converter coverage hygiene VariableContext snapshot/restore
input: typed harness contract + consultation closeout contract
output: typed execution harness implementation
```

## Observed State

```text
selected_slice=VariableContext_snapshot_restore
typed_harness_ir_contract=present
typed_harness_ir_shape_contract=present
typed_harness_ir_acceptance_bundle_contract=present
typed_harness_ir_validation_execution_bundle_contract=present
typed_harness_ir_validation_summary_artifact_contract=present
typed_harness_ir_implementation_start_boundary_contract=present
typed_harness_ir_implementation_entry_contract=present
typed_harness_ir_implementation_touch_set_contract=present
typed_harness_ir_implementation_wiring_contract=present
typed_harness_ir_implementation_patch_sequence_contract=present
crate_level_bundle_opened=1
crate_linker_opened=0
nightly_rustc_adapter_opened=0
runtime_fallback_opened=0
new_family_selection_opened=0
new_route_selection_opened=0
```

## Implementation Contract

The snapshot/restore typed execution harness must stay minimal and explicit:

- the harness stays VariableContext snapshot/restore only
- the harness uses typed Main operations instead of raw `main_lines`
- the harness does not widen route selection
- the harness does not open the nightly rustc adapter path
- the harness does not open runtime fallback
- the harness does not add new family selection

## Deferred Work

Keep these work items deferred until a later implementation slice:

- crate linker
- crate-wide MirBuilder conversion
- carrier-sensitive behavior
- returned read borrow / read-view redesign
- mutable alias redesign
- PHI / loop lowering
- Drop / unsafe / FFI
- NonAsciiOrderedKey

## Required Checks

```text
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Acceptance

```text
the VariableContext snapshot/restore derived artifact uses typed Main operations
generated Hako parse/MIR/EXE are green
main_lines is not used by VariableContext snapshot/restore
route selection remains unopened
crate linker remains unopened
nightly rustc adapter remains unopened
runtime fallback remains unopened
```

## Stop Line

```text
do_not_open_crate_linker=1
do_not_open_nightly_rustc_adapter=1
do_not_add_runtime_fallback=1
do_not_open_new_family_selection=1
do_not_open_new_route_selection=1
do_not_start_unbounded_crate_coverage=1
```
