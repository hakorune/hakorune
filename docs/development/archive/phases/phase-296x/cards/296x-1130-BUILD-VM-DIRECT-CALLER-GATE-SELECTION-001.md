Status: Done
Date: 2026-06-18
Scope: select the next direct VM import family to gate
Related:
  - docs/development/current/main/phases/phase-296x/296x-1129-BUILD-VM-TERMINAL-FAILFAST-SEAM-001.md
  - docs/development/current/main/design/build-crate-split-plan-ssot.md
  - docs/development/current/main/design/vm-active-lane-retirement-ssot.md

# BUILD-VM-DIRECT-CALLER-GATE-SELECTION-001

## Decision

```text
output_contract=build-vm-direct-caller-gate-selection-v0

selection_only=1
selected_family=runner_repl_vm_reference_gate
selected_next_task=BUILD-VM-REPL-REFERENCE-GATE-001

reason=single_public_entry_and_no_product_exe_aot_terminal_overlap
default_off_claim=0
summary=ok
```

## Candidate Ranking

```text
candidate=runner_repl_vm_reference_gate
remaining_vm_import_count=1
risk=low
selected=1

candidate=runner_common_vm_execution_and_keep_vm
remaining_vm_import_count=2
risk=medium
selected=0
reason=shared_explicit_vm_runner_family_requires_keep_route_guard_surface

candidate=joinir_runner_and_vm_bridge
remaining_vm_import_count=3
risk=medium
selected=0
reason=tests_and_dispatch_bridge_use_joinir_vm_execution_paths
```

The REPL caller is isolated behind `runner::repl::run_repl` and does not own the
product EXE/AOT terminal. It can fail fast when `vm-reference` is unavailable
without changing app/selfhost product route behavior.

## Stop Lines

```text
do_not_gate_joinir_in_repl_row=1
do_not_gate_keep_vm_in_repl_row=1
do_not_remove_vm_reference_from_default=1
do_not_claim_no_default_features_green=1
```

