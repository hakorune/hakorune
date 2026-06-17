# 296x-1054 REASON-ENUMS-VOCABULARY-DESIGN-001

Status: Landed
Date: 2026-06-17
Scope: reason enum vocabulary design

## Contract

```text
output_contract=hako-reason-enums-vocabulary-design-v0
row_kind=design

selected_option=C_lite_common_report_domain
reason_enum_merge_enabled=0
reason_domain_report_enabled=1
storage_reason_enums_kept=1
publication_reason_enum_kept=1
fastpath_deny_reason_enum_kept=1

backend_behavior_changed=0
product_default_changed=0
implementation_started=0

next_task=REASON-DOMAIN-REPORT-VOCABULARY-001
summary=ok
```

## Decision

Do not merge the reason enums.

The reason enums carry different owner semantics:

```text
GenericBoxReason / EscapeReason / DynamicReason:
  storage representation fallback owners

ObjectPublicationReason:
  publication boundary owner

LocalFastPathFallbackReason:
  eligibility denial owner
```

Merging them into one enum would make storage fallback, publication, and
fastpath eligibility look like one decision source. That would violate the
current fact/fallback separation.

## C-Lite Shape

Add a small report classification layer instead:

```text
ReasonDomain::Storage
ReasonDomain::Publication
ReasonDomain::FastPathEligibility
```

Each existing enum stays in its owning module. The shared domain is only for
audit/report vocabulary and does not replace the enum types.

## Stop Line

```text
do not merge reason enum types
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
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
