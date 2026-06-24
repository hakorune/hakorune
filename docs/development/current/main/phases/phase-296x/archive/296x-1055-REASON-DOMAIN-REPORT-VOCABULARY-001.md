# 296x-1055 REASON-DOMAIN-REPORT-VOCABULARY-001

Status: Landed
Date: 2026-06-17
Scope: reason domain report vocabulary

## Contract

```text
output_contract=hako-object-storage-plan-vocab-audit-v0
source_evidence=296x-1054
row_kind=inventory

keep_separate_count=6
merge_candidate_count=2
immediate_merge_allowed=0
vocabulary_merge_count=0
fact_fallback_separation_preserved=1
public_api_reexport_preserved=1
guard_path_compat_landed=1

reason_enum_merge_enabled=0
reason_domain_report_enabled=1
reason_domain_count=3
reason_domain_storage_enums_kept=3
reason_domain_publication_enum_kept=1
reason_domain_fastpath_enum_kept=1

first_safe_followup=OBJECT-SITE-LOCATION-VOCABULARY-DESIGN-001
summary=ok
```

## Change

Adds a report-only `ReasonDomain` classifier:

```text
StorageRepresentation
PublicationBoundary
FastPathEligibility
```

The existing reason enums remain separate. The domain classifier only explains
which owner family a reason belongs to; it does not replace the owner-specific
reason enums.

## Remaining Candidates

```text
site_location_fields
scalar_field_descriptors
```

## Stop Line

```text
do not merge reason enums
do not move publication reasons into storage
do not move fastpath deny reasons into storage
do not change backend lowering
do not change product runtime behavior
```

## Verification

```bash
cargo test -q object_storage_plan --lib
python3 -m unittest tools.hako_check.tests.test_object_storage_plan_vocab_audit
python3 tools/hako_check/object_storage_plan_vocab_audit.py --repo-root .
bash tools/checks/k2_wide_phase296x_object_storage_plan_vocab_audit_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
