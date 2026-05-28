---
Status: Landed
Date: 2026-05-29
Scope: select one facade field owner from exact-slot field inventory.
Blocker: OBJECT-LIFECYCLE-FACADE-FIELD-OWNER-SELECTION-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-227-OBJECT-LIFECYCLE-FACADE-EXACT-SLOT-FIELD-INVENTORY.md
---

# 296x-228 Object-Lifecycle Facade Field Owner Selection

## Purpose

Select one narrow owner from row227 facade field inventory before an
implementation row.

This row keeps optimization closed. It rejects generic typed-field residence
and broad method-local scalar cache because the repeated-get surface is too
small.

## Evidence

```text
output_contract=object-lifecycle-facade-field-owner-selection-v0
input_contract=object-lifecycle-facade-exact-slot-field-inventory-v0
workload_id=representative-object-lifecycle-small-block-v0
same_block_get_set_count=3
same_receiver_repeated_get_count=1
positive_net_cache_candidate_count=4
selected_owner=selected_facade_same_block_get_set_fusion
selected_reason=same_block_get_set_candidates_dominate_positive_net_surface
next_diagnostic=selected_facade_same_block_get_set_guard_surface
planned_erased_get_set_helper_calls=6
planned_added_fused_helper_calls=3
planned_net_helper_call_delta=3
planned_net_helper_call_delta_positive=1
rejected_owner=generic_typed_field_residence_retry
rejected_reason=no_family_specific_residence_plan
rejected_owner_1=facade_method_local_scalar_cache
rejected_reason_1=same_receiver_repeated_get_count_1_too_small
optimization_open=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Decision

```text
selected_owner=selected_facade_same_block_get_set_fusion
next_row=selected_facade_same_block_get_set_guard_surface
```

The selected owner is narrow: fuse same-block `field_get -> add/copy -> field_set`
patterns inside the facade family only. It should not reopen generic residence,
source rewrite, provider activation, replacement, hooks, or globals.

## Guard

```bash
bash tools/checks/k2_wide_phase296x_object_lifecycle_facade_field_owner_selection_guard.sh
```
