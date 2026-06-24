---
Status: Landed
Date: 2026-06-16
Task: OBJECT-PLAN-LOCAL-FIRST-000
Scope: Add passive ObjectPlan vocabulary for local-first representation plus
  publication sites.
Related:
  - docs/development/current/main/phases/phase-296x/296x-811-LOCAL-FIRST-OBJECT-MODEL-SSOT-001.md
  - docs/development/current/main/phases/phase-296x/296x-711-OBJECT-STORAGE-PLAN-SSOT-001.md
  - src/object_storage_plan.rs
---

# OBJECT-PLAN-LOCAL-FIRST-000

## Purpose

Define the first local-first ObjectPlan vocabulary without adding a standalone
PublicationPlan.

This row keeps the layer thin:

```text
ObjectPlan:
  representation
  publication_sites

PublicationPlan:
  not split out yet
```

No lowering, MIR JSON export, backend consumption, or runtime behavior changes
are enabled by this row.

## Report

```text
output_contract=hako-object-plan-local-first-v0
source_evidence=296x-811,296x-711
target_front=object_lifecycle_body
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1

object_plan_local_first_vocabulary_defined=1
object_plan_representation_field=ObjectStoragePlan
object_plan_publication_sites_defined=1
publication_site_reason_vocabulary_defined=1
standalone_publication_plan_enabled=0

publication_reason_plugin_or_extern=1
publication_reason_host_handle_required=1
publication_reason_dynamic_array_or_map=1
publication_reason_dynamic_nyashbox_api=1
publication_reason_return_as_dynamic_box=1
publication_reason_task_future_channel_boundary=1
publication_reason_unknown_fini_or_drop=1
publication_reason_unknown=1
unknown_publication_forces_generic_fallback=1

mirbuilder_object_management_enabled=0
mirbuilder_representation_owner=0
object_plan_execution_enabled=0
object_plan_mir_json_export_enabled=0
backend_consumes_object_plan=0
product_default_changed=0

next_task=OBJECT-PUBLICATION-INVENTORY-001
summary=ok
```

## Code Vocabulary

```text
LocalFirstObjectPlan:
  value_id
  storage: ObjectStoragePlan
  publication_sites: Vec<ObjectPublicationSite>

ObjectPublicationSite:
  value_id
  reason
  block_id
  instruction_index
```

`ObjectPublicationReason::Unknown` is conservative. Unknown publication state
must not keep a local-direct path alive.

## Stop Line

```text
do not add standalone PublicationPlan from this row
do not connect ObjectPlan to lowering from this row
do not export ObjectPlan to MIR JSON from this row
do not let backend consume ObjectPlan from this row
do not move object representation ownership into MIRBuilder
do not change product default runtime behavior
do not infer publication from helper names
```

## Handoff

The next row inventories publication sites in the target body:

```text
OBJECT-PUBLICATION-INVENTORY-001
```
