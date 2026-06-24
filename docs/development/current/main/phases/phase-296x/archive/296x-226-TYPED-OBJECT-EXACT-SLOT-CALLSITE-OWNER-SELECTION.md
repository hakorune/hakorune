---
Status: Landed
Date: 2026-05-29
Scope: choose one next owner from exact-slot callsite attribution.
Blocker: TYPED-OBJECT-EXACT-SLOT-CALLSITE-OWNER-SELECTION-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-225-TYPED-OBJECT-EXACT-SLOT-CALLSITE-ATTRIBUTION.md
---

# 296x-226 Typed-Object Exact-Slot Callsite Owner Selection

## Purpose

Choose one narrow next owner from row225 attribution before implementation.

This row keeps optimization closed. It prevents jumping back into broad MIR
residence or another page-model keeper without fresh positive evidence.

## Evidence

```text
output_contract=typed-object-exact-slot-callsite-owner-selection-v0
input_contract=typed-object-exact-slot-callsite-attribution-v0
workload_id=representative-object-lifecycle-small-block-v0
dominant_family=object_lifecycle_facade
dominant_family_pct=18.52
top_callsite_symbol=HakoAllocPageModel.acquire_usize/1
top_callsite_pct=4.54
selected_owner=object_lifecycle_facade_exact_slot_field_inventory
selected_reason=dominant_family_object_lifecycle_facade
next_diagnostic=object_lifecycle_facade_exact_slot_field_inventory
rejected_owner=page_model_followon_keeper
rejected_reason=page_model_recently_optimized_and_not_dominant_family
rejected_owner_1=generic_typed_field_residence_retry
rejected_reason_1=no_new_positive_net_helper_delta_evidence
optimization_open=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Decision

```text
selected_owner=object_lifecycle_facade_exact_slot_field_inventory
next_row=object_lifecycle_facade_exact_slot_field_inventory
```

`HakoAllocPageModel.acquire_usize/1` remains the largest single callsite, but
the dominant family is `object_lifecycle_facade`. The page-model owner was just
optimized by the RMW fusion keeper, so the next row should inventory facade
field/capsule exact-slot traffic before another implementation row.

## Guard

```bash
bash tools/checks/k2_wide_phase296x_typed_object_exact_slot_callsite_owner_selection_guard.sh
```
