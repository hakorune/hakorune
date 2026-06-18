Status: Done
Date: 2026-06-18
Scope: select the next remaining VM direct import family after REPL gate
Related:
  - docs/development/current/main/phases/phase-296x/296x-1131-BUILD-VM-REPL-REFERENCE-GATE-001.md
  - docs/development/current/main/design/build-crate-split-plan-ssot.md
  - docs/development/current/main/design/vm-active-lane-retirement-ssot.md

# BUILD-VM-DIRECT-CALLER-GATE-SELECTION-002

## Decision

```text
output_contract=build-vm-direct-caller-gate-selection-002-v0

selection_only=1
selected_family=join_ir_runner_vm_reference_gate
selected_next_task=BUILD-VM-JOINIR-RUNNER-REFERENCE-GATE-001

reason=structure_only_runner_has_small_public_api_and_is_separate_from_joinir_vm_bridge
default_off_claim=0
summary=ok
```

## Candidate Ranking

```text
candidate=join_ir_runner_vm_reference_gate
remaining_vm_import_count=2
risk=low_medium
selected=1
reason=limited_to_run_joinir_function_and_internal_execute_function

candidate=join_ir_vm_bridge_reference_gate
remaining_vm_import_count=1
risk=medium
selected=0
reason=semantic_route_b_tests_and_bridge_dispatch_use_this_path

candidate=runner_common_vm_execution_and_vm_user_factory
remaining_vm_import_count=2
risk=medium
selected=0
reason=shared_keep_vm_runner_family_requires_explicit_route_guard_surface
```

`join_ir_runner` is explicitly documented as a development / structure-only
runner. It is narrower than `join_ir_vm_bridge`, which is still used by Route B
semantic tests and dispatch helpers.

## Stop Lines

```text
do_not_gate_join_ir_vm_bridge_in_join_ir_runner_row=1
do_not_gate_keep_vm_common_helpers_in_join_ir_runner_row=1
do_not_remove_vm_reference_from_default=1
do_not_claim_no_default_features_green=1
```

