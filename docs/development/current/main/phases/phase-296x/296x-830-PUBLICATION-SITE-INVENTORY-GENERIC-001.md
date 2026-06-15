---
Status: Landed
Date: 2026-06-16
Task: PUBLICATION-SITE-INVENTORY-GENERIC-001
Scope: Inventory generic publication-site reason vocabulary from ObjectPlan code.
Related:
  - docs/development/current/main/phases/phase-296x/296x-829-ROUTEPLAN-OBJECTPLAN-HANDOFF-001.md
  - tools/allocator/hako_publication_site_generic_inventory.py
  - src/object_storage_plan.rs
---

# PUBLICATION-SITE-INVENTORY-GENERIC-001

## Purpose

`OBJECT-PUBLICATION-INVENTORY-001` was target-front specific. This row fixes
the generic publication reason vocabulary that ObjectPlan may carry, without
claiming a new source front or backend consumer.

## Report

```text
output_contract=hako-publication-site-generic-inventory-v0
source_evidence=296x-828,296x-829
source_file=src/object_storage_plan.rs
inventory_kind=code_vocabulary
publication_reason_vocabulary_count=8
publication_reason_expected_count=8
publication_reason_missing_count=0
publication_reason_extra_count=0
publication_reason_plugin_or_extern=1
publication_reason_host_handle_required=1
publication_reason_dynamic_array_or_map=1
publication_reason_dynamic_nyashbox_api=1
publication_reason_return_as_dynamic_box=1
publication_reason_task_future_channel_boundary=1
publication_reason_unknown_fini_or_drop=1
publication_reason_unknown=1
unknown_publication_forces_generic_fallback=1
standalone_publication_plan_enabled=0
objectplan_execution_enabled=0
backend_consumes_objectplan=0
product_default_changed=0
summary=ok
```

## Interpretation

The generic vocabulary is complete enough for the next backend guard row:

```text
PluginOrExternBoundary
HostHandleRequired
DynamicArrayOrMapStorage
DynamicNyashBoxApi
ReturnAsDynamicBox
TaskFutureChannelBoundary
UnknownFiniOrDrop
Unknown
```

`Unknown` stays conservative:

```text
unknown_publication_forces_generic_fallback=1
```

## Stop Line

```text
do not open a new source-front pilot from this row
do not infer publication sites from helper names
do not split standalone PublicationPlan from this row
do not let backend consume ObjectPlan from this row
do not change product default runtime behavior
```

## Handoff

```text
selected_next=BACKEND-PLAN-CONSUMER-GUARD-001
```

## Proof

```bash
bash tools/checks/k2_wide_phase296x_publication_site_generic_inventory_guard.sh
```
