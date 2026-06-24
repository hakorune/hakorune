# 296x-991 OBJECT-STORAGE-PLAN-VOCAB-AUDIT-001

Status: Landed
Date: 2026-06-17
Scope: vocabulary inventory / no merge

## Contract

```text
output_contract=hako-object-storage-plan-vocab-audit-v0
source_evidence=296x-989,296x-990,worker-audit
row_kind=inventory
keep_separate_count=6
merge_candidate_count=4
immediate_merge_allowed=0
vocabulary_merge_count=0
fact_fallback_separation_preserved=1
public_api_reexport_preserved=1
guard_path_compat_landed=1
next_task=LOCALFIRSTOBJECTPLAN-ALIAS-RETIRE-PREFLIGHT-001
summary=ok
```

## Purpose

Audit ObjectStoragePlan vocabulary after the module split.

This row does not merge types. It records which groups are real semantic
boundaries and which names are future cleanup candidates.

## Keep Separate

```text
ids:
  strong newtypes keep semantic boundaries.

storage:
  representation truth, not execution truth.

publication:
  escape / publication state, not backend facts.

local_fastpath_fact:
  positive backend-consumable permission only.

alias:
  passive alias observation, not publication state.

inventory_shadow:
  report-only surfaces, not backend proof.
```

## Merge / Retire Candidates

```text
LocalFirstObjectPlan:
  compatibility alias for ObjectPlan.
  first safe follow-up is a retire preflight, not immediate deletion.

reason_enums:
  possible synonym cluster, but domain meanings differ.
  defer.

site_location_fields:
  repeated block + instruction pair.
  defer until another consumer needs ObjectSiteLocation.

scalar_field_descriptors:
  FieldScalarPlan and FlattenedNestedFieldPlan overlap.
  defer because nested layout payload differs.
```

## Stop Line

This row does not:

```text
merge vocabulary types
remove LocalFirstObjectPlan alias
change report fields
change MIR JSON metadata
change backend lowering
move object management into MIRBuilder
```

## Verification

```bash
bash tools/checks/k2_wide_phase296x_object_storage_plan_vocab_audit_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Next

```text
LOCALFIRSTOBJECTPLAN-ALIAS-RETIRE-PREFLIGHT-001
```

The alias is the smallest safe cleanup candidate, but it needs a preflight
because old cards and guards still reference it.
