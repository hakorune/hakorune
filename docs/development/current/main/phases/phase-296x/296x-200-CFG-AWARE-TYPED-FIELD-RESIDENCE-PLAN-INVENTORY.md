---
Status: Current
Date: 2026-05-28
Scope: inventory CFG-aware typed-field residence net helper-call delta before implementation.
Blocker: CFG-AWARE-TYPED-FIELD-RESIDENCE-PLAN-INVENTORY-296X-001
Related:
  - docs/development/current/main/design/cfg-aware-typed-field-residence-ssot.md
  - tools/allocator/cfg_aware_typed_field_residence_plan.py
---

# 296x-200 CFG-Aware Typed Field Residence Plan Inventory

## Purpose

Run the selected-method CFG-aware residence plan as observation only. This row
checks whether `HakoAllocPageModel.acquire_usize/1` has positive net helper-call
erasure before any compiler/runtime transform is opened.

## Evidence

```text
output_contract=cfg-aware-typed-field-residence-plan-v0
input_contract=cfg-aware-typed-field-residence-ssot-v0
selected_method=HakoAllocPageModel.acquire_usize/1
block_count=12
eligible_resident_field_count=9
scalar_field_get_count=11
scalar_field_set_count=8
erased_field_get_count=11
erased_field_set_count=8
inserted_helper_load_count=11
inserted_helper_writeback_count=8
same_block_reused_get_count=0
coalesced_writeback_count=0
net_helper_call_delta=0
net_helper_call_delta_positive=0
cross_block_field_count=3
phi_dirty_required_count=1
phi_value_required_count=0
flush_before_call_count=0
flush_before_return_count=8
fallback_field_count=0
rejected_handle_field_count=2
implementation_recommendation=do_not_implement_cfg_aware_residence_for_selected_method
next_diagnostic=large_owner_refresh_after_residence_zero_net
transform_open=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Interpretation

The selected method has scalar field helper volume, but the current CFG-aware
policy still needs one helper load for each get and one writeback for each set.
There are no same-block duplicate gets and no coalesced writebacks, so a
selected-method residence transform would again move helpers rather than erase
them.

## Next

```text
row201:
  large_owner_refresh_after_residence_zero_net

Goal:
  refresh the large owner instead of implementing typed-field residence for
  acquire_usize/1. Candidate surfaces include ArrayBox helper cost, typed-object
  helper call boundary overhead, and another method with positive residence net.
```

Guard:

```bash
bash tools/checks/k2_wide_phase296x_cfg_aware_typed_field_residence_plan_inventory_guard.sh
```
