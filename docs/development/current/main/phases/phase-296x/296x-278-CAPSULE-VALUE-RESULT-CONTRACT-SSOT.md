---
Status: Landed
Date: 2026-05-29
Scope: define capsule ValueAggregate/materialization/writeback contract before implementation.
Blocker: CAPSULE-VALUE-RESULT-CONTRACT-SSOT-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-277-RESULT-CAPSULE-RECORD-SUCCESS-REPRESENTATION-GUARD-SURFACE.md
  - docs/development/current/main/design/capsule-value-result-contract-ssot.md
---

# 296x-278 Capsule Value Result Contract SSOT

## Purpose

Define the contract that must exist before recordSuccess can move beyond helper
fusion toward a compiler value-result representation.

This row is docs/guard only. It does not rewrite MIR, change lowering, or touch
`.hako` source.

## Evidence

```text
output_contract=capsule-value-result-contract-ssot-v0
input_contract=result-capsule-record-success-representation-guard-surface-v0
workload_id=representative-object-lifecycle-small-block-v0
contract_doc=docs/development/current/main/design/capsule-value-result-contract-ssot.md
representation_before=ExactSlotObject
representation_after=ValueAggregateDelta
public_capsule_object_preserved=1
hot_update_value_delta_allowed=1
observer_materialization_required=1
same_module_method_required=1
receiver_capsule_type_known_required=1
receiver_slot_plan_known_required=1
internal_call_count_required=0
unknown_escape_required=0
stored_into_other_object_required=0
returned_as_object_required=0
all_observer_boundaries_known_required=1
materialization_policy_known_required=1
net_helper_delta_positive_required=1
selected_next=capsule_value_result_plan_inventory
rejected_owner=record_success_helper_fusion_implementation
rejected_reason=value_result_plan_inventory_required_before_lowering
rejected_owner_1=public_capsule_object_erasure
rejected_reason_1=observer_state_identity_must_be_preserved
optimization_open=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Decision

```text
selected_next=capsule_value_result_plan_inventory
next_row=capsule_value_result_plan_inventory
optimization_open=0
```

The next row must inventory whether recordSuccess can produce a positive-net
`CapsuleValueResultPlan` with known observer/materialization boundaries. If it
cannot, the implementation path must remain exact-slot helper based.

## Guard

```bash
bash tools/checks/k2_wide_phase296x_capsule_value_result_contract_ssot_guard.sh
```
