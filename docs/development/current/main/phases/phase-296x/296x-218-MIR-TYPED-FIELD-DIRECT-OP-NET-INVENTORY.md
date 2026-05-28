---
Status: Landed
Date: 2026-05-29
Scope: inventory positive net helper-call delta for typed-field direct-op candidates before implementation.
Blocker: MIR-TYPED-FIELD-DIRECT-OP-NET-INVENTORY-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-217-POST-EXACT-SLOT-DIRECT-HELPER-OWNER-REFRESH.md
  - docs/development/current/main/phases/phase-296x/296x-193-MIR-TYPED-FIELD-RESIDENCE-SSOT.md
---

# 296x-218 MIR Typed-Field Direct-Op Net Inventory

## Purpose

Count whether the current exact-slot typed-object helper calls can be erased by
a later direct-op lowering row with positive net helper-call delta.

This row is inventory-only. It does not reopen typed-field residence and does
not implement a transform.

## Decision

```text
selected_owner_family=mir_typed_field_direct_op_inventory
selected_method=HakoAllocPageModel.acquire_usize/1
selected_next=mir_typed_field_direct_op_guard_surface
```

The inventory uses typed-object slot plans and MIR receiver type metadata rather
than `declared_type` alone. This keeps `declared_type=None` call sites visible
when the C-ABI lowering still has an exact typed-object plan.

## Evidence

```text
output_contract=mir-typed-field-direct-op-net-inventory-v0
input_contract=typed-object-exact-slot-owner-refresh-v0
workload_id=representative-object-lifecycle-small-block-v0
hot_method_count=5
missing_hot_method_count=0
typed_object_plan_box_count=10
field_get_total=39
field_set_total=41
eligible_field_get_count=39
eligible_field_set_count=41
eligible_i64_count=17
eligible_u64_count=0
eligible_usize_count=50
eligible_handle_count=13
unknown_receiver_count=0
unsupported_storage_count=0
projected_erased_exact_helper_call_count=80
projected_added_helper_call_count=0
projected_net_helper_call_delta=80
dynamic_projected_net_helper_call_delta=30072832
residence_writeback_required_count=41
dynamic_residence_writeback_required_count=11689984
barrier_unknown_call_count=7
barrier_phi_count=21
barrier_return_count=16
projected_exact_helper_symbol_0=nyash.object.exact_slot_get_handle_hii
projected_exact_helper_symbol_0_count=11
projected_exact_helper_symbol_1=nyash.object.exact_slot_get_i64_hii
projected_exact_helper_symbol_1_count=10
projected_exact_helper_symbol_2=nyash.object.exact_slot_get_u64_hii
projected_exact_helper_symbol_2_count=18
projected_exact_helper_symbol_3=nyash.object.exact_slot_set_handle_hii
projected_exact_helper_symbol_3_count=2
projected_exact_helper_symbol_4=nyash.object.exact_slot_set_i64_hii
projected_exact_helper_symbol_4_count=7
projected_exact_helper_symbol_5=nyash.object.exact_slot_set_u64_hiu
projected_exact_helper_symbol_5_count=32
selected_method=HakoAllocPageModel.acquire_usize/1
selected_method_net_helper_call_delta=21
selected_method_dynamic_net_helper_call_delta=11010048
selected_field_by_dynamic_net=HakoAllocPageModel.reject_count.usize
selected_field_by_dynamic_net_dynamic_count=3153920
inventory_only=1
projected_net_helper_call_delta_positive=1
dynamic_projected_net_helper_call_delta_positive=1
selected_method_required=1
projected_exact_helper_symbol_coverage_matches_mir_storage_counts=1
residence_inserted_load_writeback_delta_used=0
residence_transform_open=0
direct_op_transform_open=0
previous_residence_zero_net_guard=1
selected_next=mir_typed_field_direct_op_guard_surface
by_name_special_case=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Guard

```bash
bash tools/checks/k2_wide_phase296x_mir_typed_field_direct_op_net_inventory_guard.sh
```
