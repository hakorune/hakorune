---
Status: Landed
Date: 2026-06-15
Task: AGG-STORAGE-PLAN-000
Scope: Introduce passive AggregateStoragePlan vocabulary.
Related:
  - src/aggregate_storage_plan.rs
  - docs/development/current/main/design/record-box-two-surface-one-substrate-ssot.md
---

# AGG-STORAGE-PLAN-000

## Result

```text
output_contract=hako-aggregate-storage-plan-v0
record_box_surface_model=two_surface_one_substrate
aggregate_storage_plan_vocabulary_defined=1
aggregate_storage_plan_execution_enabled=0
aggregate_subject_record_enabled=1
aggregate_subject_enum_payload_enabled=1
aggregate_subject_tuple_payload_enabled=1
aggregate_subject_closure_env_enabled=1
object_storage_plan_shared_substrate=1
mirbuilder_representation_owner=0
product_default_changed=0
selected_next=AGG-OBJECT-STORAGE-BRIDGE-001
summary=ok
```

## Stop Line

```text
do not lower records through this row
do not mutate MIR
do not collapse record and box source semantics
do not enable backend aggregate lowering
```
