---
Status: Landed
Date: 2026-06-15
Task: RECORD-BOX-SURFACE-000
Scope: Land the user-facing `record` / `box` two-surface decision.
Related:
  - docs/development/current/main/design/record-box-two-surface-one-substrate-ssot.md
---

# RECORD-BOX-SURFACE-000

## Result

```text
output_contract=hako-record-box-surface-v0
record_box_surface_model=two_surface_one_substrate
record_identity_free_value_surface=1
box_identity_behavior_lifecycle_surface=1
source_surface_collapsed_to_box=0
record_methods_enabled=0
ordinary_box_with_enabled=0
automatic_record_to_box_copy=0
aggregate_storage_plan_shared_substrate=1
object_storage_plan_shared_substrate=1
mirbuilder_representation_owner=0
selected_next=RECORD-BOX-DOCS-001
summary=ok
```

## Stop Line

```text
do not remove record
do not collapse record and box into one source model
do not describe record as a fast box
do not move representation choice into MIRBuilder
```
