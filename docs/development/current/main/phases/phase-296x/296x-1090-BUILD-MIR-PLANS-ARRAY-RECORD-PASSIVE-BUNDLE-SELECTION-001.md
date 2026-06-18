Status: Done
Date: 2026-06-18
Scope: select ArrayRecord passive data bundle for hakorune-mir-plans split
Related:
  - docs/development/current/main/design/build-crate-split-plan-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-1089-BUILD-MIR-PLANS-TYPED-FIELD-STORAGE-SPLIT-001.md
  - src/mir/function/object_metadata.rs

# BUILD-MIR-PLANS-ARRAY-RECORD-PASSIVE-BUNDLE-SELECTION-001

## Purpose

Select the next passive bundle after `TypedObjectFieldStorage` moved into
`hakorune-mir-plans`.

## Decision

Move the record-layout / ArrayRecord / PackedArray passive metadata rows as one
bundle.

```text
selected_family=array_record_passive_bundle
move=RecordLayoutPlan,ArrayRecordStoragePlan,ArrayRecordAutoUseEligibilityPlan,ArrayRecordMaterializationBoundaryPlan,ArrayRecordPackedAutoUsePilotPlan,SourcePackedArrayAutoUsePilotPlan,SourcePackedArrayDirectReadConsumptionPlan,HakoAllocPackedStorePilotPlan
keep_main_crate=record layout refresh, array record refresh, eligibility classification, materialization boundary classification, packed pilot producers
behavior_changed=0
```

The bundle depends only on strings, integers, and `TypedObjectFieldStorage`.
It does not need `MirModule`, `MirFunction`, `MirInstruction`, or backend
emitters.

## Contract

```text
output_contract=build-mir-plans-array-record-passive-bundle-selection-v0

selected_family=array_record_passive_bundle
boxshape_only=1
boxcount_allowed=0
behavior_change_allowed=0
producer_logic_moved=0
backend_lowering_enabled=0
runtime_route_enabled=0

summary=ok
```

## Next

```text
next_task=BUILD-MIR-PLANS-ARRAY-RECORD-PASSIVE-BUNDLE-SPLIT-001
```
