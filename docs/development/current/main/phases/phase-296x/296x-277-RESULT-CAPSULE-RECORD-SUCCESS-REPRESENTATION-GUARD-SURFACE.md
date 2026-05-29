---
Status: Landed
Date: 2026-05-29
Scope: freeze recordSuccess representation guard before any helper fusion implementation.
Blocker: RESULT-CAPSULE-RECORD-SUCCESS-REPRESENTATION-GUARD-SURFACE-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-276-RESULT-CAPSULE-OWNER-SELECTION-AFTER-RELEASE-KNOWN-LIVE-ROLLBACK.md
  - docs/development/current/main/design/result-capsule-value-representation-ssot.md
---

# 296x-277 Result Capsule RecordSuccess Representation Guard Surface

## Purpose

Reframe the selected recordSuccess owner as a representation decision rather
than a helper-fusion implementation row.

This row is docs/guard only. It keeps optimization closed and selects the
capsule value-result contract SSOT before any implementation.

## Evidence

```text
output_contract=result-capsule-record-success-representation-guard-surface-v0
input_contract=result-capsule-owner-selection-after-release-known-live-rollback-v0
workload_id=representative-object-lifecycle-small-block-v0
selected_methods=HakoAllocObjectLifecycleAllocResult.recordSuccess/1,HakoAllocObjectLifecycleReleaseResult.recordSuccess/2
current_representation=ExactSlotObject
candidate_representation_0=FusedHelper
candidate_representation_1=ValueAggregate
alloc_record_success_field_op_count=8
release_record_success_field_op_count=6
record_success_combined_field_op_count=14
alloc_record_success_copy_count=9
release_record_success_copy_count=3
alloc_record_success_branch_count=2
release_record_success_branch_count=0
internal_call_count=0
identity_observed=1
unknown_escape=0
stored_into_other_object=0
returned_as_object=0
observer_boundary_count=4
materialization_required=1
helper_fusion_erased_helper_calls=14
helper_fusion_added_helper_calls=2
helper_fusion_net_delta=12
helper_fusion_net_delta_positive=1
value_aggregate_net_delta_known=0
value_aggregate_requires_contract=1
selected_next=capsule_value_result_contract_ssot
selected_reason=helper_fusion_is_positive_but_capsule_representation_contract_is_missing
rejected_owner=record_success_helper_fusion_implementation
rejected_reason=implementation_before_value_representation_contract_would_lock_in_helper_world
rejected_owner_1=generic_typed_field_residence_retry
rejected_reason_1=no_capsule_specific_materialization_policy
rejected_owner_2=source_inline_success_result_fast_path
rejected_reason_2=prior_source_inline_success_result_regressed_and_was_rolled_back
optimization_open=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Decision

```text
selected_next=capsule_value_result_contract_ssot
next_row=capsule_value_result_contract_ssot
optimization_open=0
```

The helper-fusion delta is positive, but implementing it now would continue the
helper-world trajectory. The cleaner compiler shape needs a capsule
value-result contract first: what counts as identity, when observer state must
materialize, and where writeback is required.

## Guard

```bash
bash tools/checks/k2_wide_phase296x_result_capsule_record_success_representation_guard_surface_guard.sh
```
