Status: Done
Date: 2026-06-18
Scope: select the next remaining VM direct import family after JoinIR runner gate
Related:
  - docs/development/current/main/phases/phase-296x/296x-1133-BUILD-VM-JOINIR-RUNNER-REFERENCE-GATE-001.md
  - docs/development/current/main/design/build-crate-split-plan-ssot.md
  - docs/development/current/main/design/vm-active-lane-retirement-ssot.md

# BUILD-VM-DIRECT-CALLER-GATE-SELECTION-003

## Decision

```text
output_contract=build-vm-direct-caller-gate-selection-003-v0

selection_only=1
selected_family=join_ir_vm_bridge_reference_gate
selected_next_task=BUILD-VM-JOINIR-BRIDGE-REFERENCE-GATE-001

reason=single_public_run_joinir_via_vm_entry_can_fail_fast_without_retyping_bridge_conversion
default_off_claim=0
summary=ok
```

## Candidate Ranking

```text
candidate=join_ir_vm_bridge_reference_gate
remaining_vm_import_count=1
risk=low_medium
selected=1
reason=run_joinir_via_vm_can_keep_signature_and_return_explicit_error_without_vm-reference

candidate=runner_common_vm_execution_and_vm_user_factory
remaining_vm_import_count=2
risk=medium
selected=0
reason=shared_keep_vm_runner_family_requires_route-level gate surface
```

The bridge conversion modules can remain available without VM execution. Only
the `run_joinir_via_vm` terminal path needs a `vm-reference` branch.

## Stop Lines

```text
do_not_gate_bridge_conversion_modules=1
do_not_gate_keep_vm_common_helpers_in_bridge_row=1
do_not_remove_vm_reference_from_default=1
do_not_claim_no_default_features_green=1
```

