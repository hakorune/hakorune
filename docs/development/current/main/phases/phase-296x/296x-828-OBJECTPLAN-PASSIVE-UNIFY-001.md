---
Status: Landed
Date: 2026-06-16
Task: OBJECTPLAN-PASSIVE-UNIFY-001
Scope: Make `ObjectPlan` the canonical passive vocabulary while preserving local-first compatibility.
Related:
  - docs/development/current/main/design/compiler-object-final-shape-ssot.md
  - docs/development/current/main/design/object-storage-plan-boundary-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-827-SELFHOST-MIR-OBJECT-METADATA-001.md
---

# OBJECTPLAN-PASSIVE-UNIFY-001

## Purpose

Previous local-first rows introduced `LocalFirstObjectPlan` as passive
vocabulary. The final compiler object-shape contract uses `ObjectPlan` as the
owner name for representation plus publication-site truth.

This row unifies the code vocabulary without changing behavior.

## Result

```text
output_contract=hako-objectplan-passive-unify-v0
source_evidence=296x-825,296x-827
objectplan_canonical_vocabulary_defined=1
objectplan_struct_name=ObjectPlan
objectplan_storage_field=ObjectStoragePlan
objectplan_publication_sites_field=Vec<ObjectPublicationSite>
objectplan_is_representation_truth=1
objectplan_is_publication_site_truth=1
local_first_object_plan_compat_alias_enabled=1
standalone_publication_plan_enabled=0
objectplan_execution_enabled=0
backend_consumes_objectplan=0
mirbuilder_object_management_enabled=0
product_default_changed=0
selected_next=ROUTEPLAN-OBJECTPLAN-HANDOFF-001
summary=ok
```

## Compatibility

`LocalFirstObjectPlan` remains as a compatibility alias for older phase cards
and guards. New rows should use `ObjectPlan`.

```text
canonical_name=ObjectPlan
compat_alias=LocalFirstObjectPlan
compat_alias_retire_condition=old local-first rows no longer referenced by active guards
```

## Stop Line

```text
do not enable ObjectPlan execution in this row
do not make backend consume ObjectPlan in this row
do not split standalone PublicationPlan in this row
do not remove LocalFirstObjectPlan compatibility alias in this row
do not move object representation ownership into MIRBuilder
```

## Proof

```bash
bash tools/checks/k2_wide_phase296x_objectplan_passive_unify_guard.sh
cargo test --lib object_storage_plan
```
