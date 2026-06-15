---
Status: Landed
Date: 2026-06-15
Task: MANUAL-SYNC-RECORD-BOX-001
Scope: Synchronize user-facing manual/readme entry points with the current
  record / box two-surface one-substrate decision.
Related:
  - docs/development/current/main/design/record-box-two-surface-one-substrate-ssot.md
  - docs/reference/language/types.md
  - docs/reference/boxes-system/README.md
  - docs/reference/boxes-system/everything-is-box.md
---

# MANUAL-SYNC-RECORD-BOX-001

## Result

```text
output_contract=hako-manual-sync-record-box-v0
source_evidence=296x-738
record_box_surface_model=two_surface_one_substrate
readme_box_first_architecture_removed=1
readme_everything_is_box_slogan_removed=1
boxes_system_historical_banner_added=1
record_methods_disabled_reference_visible=1
object_storage_plan_reference_linked=1
aggregate_storage_plan_reference_linked=1
ordinary_box_with_enabled=0
summary=ok
```

## Decision

Manual/readme entry points must not teach the old “Everything is Box” model as
the whole language model.

Current wording:

```text
record:
  identity-free value aggregate

box:
  identity / behavior / lifecycle boundary

internal optimization:
  AggregateStoragePlan / ObjectStoragePlan may share backend substrate where
  proof permits
```

## Stop Line

```text
do not remove legacy pages
do not make legacy pages current truth
do not describe record as a faster box
do not treat record semantics as proof for boxes
```
