# Capsule Value Result Contract SSOT

Status: Active
Date: 2026-05-29
Scope: hako_alloc result-capsule ValueAggregate contract before MIR/backend implementation.

## Purpose

Define when a result capsule update can move from runtime exact-slot helper
traffic into a compiler value-result representation.

This is the first contract row after the recordSuccess representation guard.
It does not implement a transform. It defines the shape that later inventory,
planner, and lowering rows must prove before erasing helper calls.

## Key Decision

Result capsules are not globally erased.

```text
public capsule object:
  remains visible for observer methods and facade-owned state

hot update operation:
  may be represented as a ValueResult delta when all consumers and
  materialization points are known
```

This avoids pretending that `HakoAllocObjectLifecycleAllocResult` or
`HakoAllocObjectLifecycleReleaseResult` has no identity. The capsule object has
observable state. The optimization target is the hot update shape, not the
public object contract.

## CapsuleValueResultPlan

```text
CapsuleValueResultPlan {
  capsule_type
  selected_method
  receiver_value
  representation_before = ExactSlotObject
  representation_after = ValueAggregateDelta

  components:
    last_page_id
    last_block_id
    last_reason
    last_ok
    success_count_delta
    failure_count_delta
    reusable_success_count_delta
    active_success_count_delta

  branch_inputs:
    selected_kind

  materialization_policy:
    writeback_before_observer
    writeback_before_unknown_escape
    writeback_before_return_if_public_state_changed

  fallback_policy:
    keep_exact_slot_helpers
}
```

`ValueAggregateDelta` means the compiler carries the result update as values
inside the selected region. It must not silently drop observer state updates.

## Eligibility

A method is eligible only when all conditions hold:

```text
same_module_method=1
receiver_capsule_type_known=1
receiver_slot_plan_known=1
internal_call_count=0
unknown_escape=0
stored_into_other_object=0
returned_as_object=0
all_observer_boundaries_known=1
materialization_policy_known=1
net_helper_delta_positive=1
```

If any condition is not proven, the row must keep the exact-slot helper path.

## Observer Boundaries

Observer boundaries are reads of public capsule state, including:

```text
pageId()
blockId()
reason()
ok()
counter observer methods if introduced
facade methods that return the capsule object
unknown calls that may observe the capsule
```

The first contract implementation must treat observer boundaries as
materialization barriers. It may later refine this with dominance and liveness,
but the default is conservative.

## RecordSuccess Current Shape

```text
alloc_method=HakoAllocObjectLifecycleAllocResult.recordSuccess/1
alloc_shape=branch_aware
alloc_field_ops=8
alloc_copy_ops=9
alloc_branch_count=2

release_method=HakoAllocObjectLifecycleReleaseResult.recordSuccess/2
release_shape=straightline
release_field_ops=6
release_copy_ops=3
release_branch_count=0

combined_field_ops=14
internal_call_count=0
helper_fusion_net_delta=12
value_aggregate_net_delta_known=0
```

Because `value_aggregate_net_delta_known=0`, the next row must be an inventory
or planner row, not a lowering row.

## Required Row Order

```text
1. representation guard surface
2. capsule value-result contract SSOT
3. value-result plan inventory
4. selected plan guard surface
5. implementation
6. measurement
7. owner refresh
```

Rows 3 and 4 may collapse only if the inventory produces a single unambiguous
plan and all guard counters are present.

## Non-Goals

- Removing public capsule objects.
- Source-level inline success result fast path.
- Generic typed-field residence retry.
- Generic MIR CSE.
- By-name hako_alloc special cases.
- Silent fallback from a selected ValueAggregate plan.
- Provider activation, allocator replacement, hooks, or global allocator.

## Fail-Fast Boundary

```text
declared_value_result_plan=1
and materialization_policy_known=0
  -> fail-fast

declared_value_result_plan=1
and net_helper_delta_positive=0
  -> fail-fast

unproven method shape
  -> no plan, keep exact-slot helper path
```

The contract is conservative: unproven cases stay on the existing helper path.
