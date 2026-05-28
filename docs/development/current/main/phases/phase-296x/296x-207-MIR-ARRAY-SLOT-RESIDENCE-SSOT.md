---
Status: Current
Date: 2026-05-28
Scope: define ArraySlotResidencePlan / DirectSlotOp after the Array runtime backend floor measurement.
Blocker: MIR-ARRAY-SLOT-RESIDENCE-SSOT-296X-001
Related:
  - docs/development/current/main/design/mir-array-slot-residence-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-206-ARRAY-RUNTIME-SINGLE-THREAD-STORE-BACKEND-KEEPER-MEASUREMENT.md
---

# 296x-207 MIR Array Slot Residence SSOT

## Purpose

Define the next C-parity seam after row206 proved that removing the ArrayBox
runtime storage lock boundary is a keeper. Runtime `SingleThreadExact` is a
floor measurement; the long-term target is to erase hot ArrayBox get/set helper
calls in selected MIR methods when the array identity, storage kind, index
range, and writeback barriers are proven.

## Decision

```text
Decision: provisional

mir_array_slot_residence_ssot=accepted
design_ssot=docs/development/current/main/design/mir-array-slot-residence-ssot.md
runtime_array_backend_floor=measured
array_helper_abi_fallback=1
transform_open=0
positive_net_helper_call_delta_required=1
by_name_hako_alloc_special_case=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Planned Work

```text
row208:
  mir_array_slot_residence_inventory

Goal:
  count eligible ArrayBox get/set helper erasure, added guards/writebacks,
  barriers, and net helper-call delta before any transform.
```

## Acceptance

```text
mir_array_slot_residence_ssot=accepted
transform_open=0
array_helper_abi_fallback=1
positive_net_helper_call_delta_required=1
by_name_hako_alloc_special_case=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

Guard:

```bash
bash tools/checks/k2_wide_phase296x_mir_array_slot_residence_ssot_guard.sh
```
