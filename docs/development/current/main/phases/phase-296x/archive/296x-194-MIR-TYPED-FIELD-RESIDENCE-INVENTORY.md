---
Status: Landed
Date: 2026-05-28
Scope: inventory MIR typed-field residence candidates before any transform.
Blocker: MIR-TYPED-FIELD-RESIDENCE-INVENTORY-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-193-MIR-TYPED-FIELD-RESIDENCE-SSOT.md
---

# 296x-194 MIR Typed Field Residence Inventory

## Purpose

Count typed-field residence candidates in the selected object-lifecycle hot
methods before opening any MIR transform. Row192 showed the runtime storage
fast lane is a keeper, but C parity still requires removing exported
field-helper calls from hot scalar field access. This row only inventories that
surface.

## Inventory Boundary

```text
candidate:
  field_get / field_set with scalar declared_type

not_candidate_yet:
  handle fields
  missing or dynamic declared_type
  ArrayBox runtime helper surface

barrier:
  mir_call with effects
  phi
  return
  missing/dynamic declared_type
```

The inventory is intentionally conservative enough to avoid by-name hako_alloc
special cases, while still showing which method has the largest helper-erasure
surface.

## Output Contract

```text
output_contract=mir-typed-field-residence-inventory-v0
input_contract=mir-typed-field-residence-ssot-v0
workload_id=representative-object-lifecycle-small-block-v0
hot_method_count=...
eligible_field_get_count=...
eligible_field_set_count=...
would_erase_helper_call_count=...
required_writeback_count=...
barrier_unknown_call_count=...
barrier_phi_count=...
barrier_return_count=...
barrier_dynamic_slot_count=...
selected_method=...
selected_method_eligible_total=...
selected_method_dynamic_eligible_estimate=...
transform_open=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Acceptance

```text
inventory_contract=accepted
transform_open=0
selected_method_required=1
would_erase_helper_call_count_positive=1
by_name_special_case=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```
