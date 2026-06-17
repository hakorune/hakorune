# 296x-1053 OBJECT-STORAGE-VOCAB-AUDIT-STALE-ROW-CLEANUP-001

Status: Landed
Date: 2026-06-17
Scope: remove retired LocalFirstObjectPlan from vocabulary audit candidates

## Contract

```text
output_contract=hako-object-storage-plan-vocab-audit-v0
source_evidence=296x-994,296x-1050,296x-1052
row_kind=inventory

keep_separate_count=6
merge_candidate_count=3
immediate_merge_allowed=0
vocabulary_merge_count=0
fact_fallback_separation_preserved=1
public_api_reexport_preserved=1
guard_path_compat_landed=1

local_first_object_plan_alias_retired=1
exact_stack_object_retired=1
fastpath_reachability_rust_vocab_retired=1
first_safe_followup=REASON-ENUMS-VOCABULARY-DESIGN-001
summary=ok
```

## Change

The `LocalFirstObjectPlan` compatibility alias was already retired in
`296x-994`. Keeping it as the first merge candidate made the current audit
point at completed work.

This row removes that stale audit row. The remaining merge candidates are:

```text
reason_enums
site_location_fields
scalar_field_descriptors
```

## Stop Line

```text
do not change object planning behavior
do not remove historical phase cards
do not merge reason enums in this row
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
