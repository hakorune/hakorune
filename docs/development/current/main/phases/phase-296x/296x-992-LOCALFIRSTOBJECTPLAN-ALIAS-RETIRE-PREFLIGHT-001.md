# 296x-992 LOCALFIRSTOBJECTPLAN-ALIAS-RETIRE-PREFLIGHT-001

Status: Landed
Date: 2026-06-17
Scope: alias-retire preflight / no alias removal

## Contract

```text
output_contract=hako-localfirstobjectplan-alias-retire-preflight-v0
source_evidence=296x-828,296x-991,worker-rg-audit
row_kind=preflight
preflight_input_exact_token_reference_count=27
live_api_compat_reference_count=5
historical_doc_reference_count=8
guard_expectation_reference_count=7
mirbuilder_forbidden_term_guard_preserved=1
public_alias_currently_enabled=1
report_field_currently_enabled=1
historical_guard_requires_alias_count=3
immediate_alias_removal_allowed=0
vocabulary_merge_count=0
backend_lowering_changed=0
mirbuilder_object_management_enabled=0
smallest_safe_next=LOCALFIRSTOBJECTPLAN-ALIAS-RETIRE-GUARD-COMPAT-001
summary=ok
```

## Purpose

Preflight the `LocalFirstObjectPlan` compatibility alias before any removal.

The previous vocabulary audit identified the alias as the smallest cleanup
candidate, but current code, reports, and historical guards still depend on the
compatibility spelling. This row records those dependencies and explicitly
blocks direct alias removal.

## Findings

```text
Live API compatibility:
  src/object_storage_plan/storage.rs defines:
    pub type LocalFirstObjectPlan = ObjectPlan
  src/object_storage_plan.rs re-exports storage::*.
  src/object_storage_plan/tests.rs still exercises LocalFirstObjectPlan::new.
  report.rs publishes local_first_object_plan_compat_alias_enabled=1.

Historical / current docs:
  object-storage-plan-boundary-ssot.md describes LocalFirstObjectPlan as a
  compatibility alias.
  296x-812 introduced the original local-first vocabulary.
  296x-828 preserved the alias while canonicalizing ObjectPlan.
  296x-991 records the alias as a retire candidate, not an immediate deletion.

Guard expectations:
  object_plan_local_first_guard accepts struct or alias.
  objectplan_passive_unify_guard requires the alias.
  routeplan_objectplan_handoff_guard requires the alias.
  mirbuilder_object_boundary_guard forbids the term under src/mir/builder.
```

## Stop Line

This row does not:

```text
remove LocalFirstObjectPlan alias
change object storage public re-exports
change report fields
change MIR JSON metadata
change backend lowering
move object management into MIRBuilder
merge vocabulary types
```

## Next

```text
LOCALFIRSTOBJECTPLAN-ALIAS-RETIRE-GUARD-COMPAT-001
```

That row should make historical guards and reports tolerate alias retirement
while preserving historical traceability. Actual alias removal remains a later
row.

## Verification

```bash
bash tools/checks/k2_wide_phase296x_localfirstobjectplan_alias_retire_preflight_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
