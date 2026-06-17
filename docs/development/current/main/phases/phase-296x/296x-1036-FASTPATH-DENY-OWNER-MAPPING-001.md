Status: Done
Date: 2026-06-17
Scope: map fastpath deny reasons to the next owner lane.
Related:
  - docs/development/current/main/design/fastpath-eligibility-resolver-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-1035-FASTPATH-DECISION-VOCAB-000.md

# FASTPATH-DENY-OWNER-MAPPING-001

## Purpose

Prevent `Deny(reason)` from becoming dead diagnostic vocabulary.

Every passive deny reason now has an owner mapping so the next row can move by
owner evidence instead of guessing from a failed fastpath.

## Change

Added:

```text
FastPathDenyOwner
LocalFastPathFallbackReason::owner_mapping()
```

Expanded passive deny reason vocabulary to include resolver-specific cases:

```text
UnknownValue
RoutePlanMissing
ObjectPlanMissing
CycleDetected
PhiMergeNotProven
LoopCarriedNotProven
InterprocSummaryMissing
```

Existing behavior is unchanged. These reasons are passive vocabulary only.

## Owner Mapping

```text
OpenWorld -> route_open_world_boundary
UnknownValue -> value_origin_inventory
AliasUnknown -> alias_classifier
PublishedBeforeSite -> publication_classifier
MaybePublishedBeforeSite -> publication_classifier_or_phi_freshness
RoutePlanMissing -> route_proof_producer
DynamicRoute -> routeplan_boxcallable_registry
ObjectPlanMissing -> objectplan_producer
GenericStorage -> object_storage_plan_producer
BackendMissing -> backend_consumer_seam
CycleDetected -> recursive_dependency_inventory
PhiMergeNotProven -> phi_lifecycle_alias_freshness
LoopCarriedNotProven -> loop_carried_proof_lane
InterprocSummaryMissing -> call_summary_lane
UnknownCall -> call_summary_lane
```

## Validation

```text
cargo test -q object_storage_plan --lib
cargo test -q build_mir_json_root_emits_local_fastpath_facts --lib
```

Both passed.

## Contract

```text
output_contract=fastpath-deny-owner-mapping-v0
fastpath_deny_owner_mapping_defined=1
fastpath_deny_without_owner_allowed=0
resolver_execution_enabled=0
backend_behavior_changed=0
route_priority_changed=0
implementation_scope=passive_vocabulary_only
next_task=FASTPATH-ALIAS-PUBLICATION-MVP-001
summary=ok
```

## Stop Lines

```text
do not make deny reasons backend-consumable
do not turn fallback evidence into facts
do not run resolver execution in this row
do not change route priority
```
