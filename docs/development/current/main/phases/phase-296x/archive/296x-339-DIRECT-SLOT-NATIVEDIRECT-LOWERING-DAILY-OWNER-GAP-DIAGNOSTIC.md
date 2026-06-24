---
Status: Landed
Date: 2026-05-29
Scope: diagnose the exact-EXE measurement blocker after the Python selected-method lowering pilot.
Blocker: DIRECT-SLOT-NATIVEDIRECT-LOWERING-SELECTED-METHOD-MEASUREMENT-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-338-DIRECT-SLOT-NATIVEDIRECT-LOWERING-SELECTED-METHOD-PILOT.md
  - crates/nyash-llvm-compiler/README.md
  - src/llvm_py/README.md
---

# 296x-339 Direct Slot NativeDirect Lowering Daily-Owner Gap Diagnostic

## Purpose

Classify why the row338 selected-method lowering pilot cannot be accepted by
the object-lifecycle exact-EXE measurement yet.

The row338 implementation landed in `src/llvm_py/**`, but the current
exact-EXE daily route is `ny-llvmc`'s boundary route. The real measurement
therefore did not consume the Python lowering hook and still emitted existing
exact-slot helper calls for the selected method.

## Diagnostic Contract

```text
output_contract=direct-slot-nativedirect-lowering-daily-owner-gap-diagnostic-v0
input_contract=direct-slot-nativedirect-lowering-selected-method-pilot-v0
attempted_measurement=object_lifecycle_exact_exe_direct_slot_exact
attempted_typed_object_store=direct_slot_exact
attempted_array_slot_store=single_thread_exact
daily_exact_exe_owner=ny_llvmc_boundary_route
row338_owner=llvmlite_keep_lane_field_access_py
daily_owner_gap_detected=1
observed_failure=exact_exe_trap_before_semantic_report
failure_reason=boundary_route_still_emits_exact_slot_helpers_for_direct_slot_handles
python_lowering_is_not_daily_owner=1
measurement_acceptance=blocked
selected_next=boundary_route_selected_method_nativedirect_lowering_pilot
provider_activation=0
host_replacement=0
replacement_active=0
hook_installed=0
global_allocator=0
winner_claim=0
summary=ok
```

## Decision

```text
selected_owner_family=ny_llvmc_boundary_route_selected_method_nativedirect_lowering
selected_reason=mainline exact-EXE measurement must consume boundary-owned lowering, not llvmlite keep-lane lowering
optimization_open=0
```

The next row must implement the same selected-method DirectSlot payload
load/store shape in the boundary route before retrying exact-EXE measurement.

## Guard

```bash
bash tools/checks/k2_wide_phase296x_direct_slot_nativedirect_lowering_daily_owner_gap_diagnostic_guard.sh
```
