# 296x-1056 SITE-LOCATION-AUDIT-STALE-ROW-CLEANUP-001

Status: Landed
Date: 2026-06-17
Scope: remove completed ObjectSiteLocation migration from audit candidates

## Contract

```text
output_contract=hako-object-storage-plan-vocab-audit-v0
source_evidence=296x-996,296x-998,296x-999
row_kind=inventory

keep_separate_count=6
merge_candidate_count=1
immediate_merge_allowed=0
vocabulary_merge_count=0
fact_fallback_separation_preserved=1
public_api_reexport_preserved=1
guard_path_compat_landed=1

object_site_location_field_migration_complete=1
site_location_fields_candidate_retired=1
first_safe_followup=SCALAR-FIELD-DESCRIPTOR-VOCABULARY-DESIGN-001
summary=ok
```

## Change

`ObjectSiteLocation` already owns the block/instruction pair in current
`object_storage_plan` rows:

```text
ObjectPublicationSite.location
LocalPublicationInventoryRow.location
LocalFastPathFact.location
```

The remaining `block_id()` / `instruction_index()` accessors are compatibility
read accessors, not duplicate storage truth. This row removes the stale
`site_location_fields` merge candidate from the current audit.

## Remaining Candidate

```text
scalar_field_descriptors
```

## Stop Line

```text
do not remove ObjectSiteLocation
do not remove compatibility accessors in this row
do not change object planning behavior
do not change backend lowering
```

## Verification

```bash
python3 -m unittest tools.hako_check.tests.test_object_storage_plan_vocab_audit
python3 tools/hako_check/object_storage_plan_vocab_audit.py --repo-root .
bash tools/checks/k2_wide_phase296x_object_storage_plan_vocab_audit_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
