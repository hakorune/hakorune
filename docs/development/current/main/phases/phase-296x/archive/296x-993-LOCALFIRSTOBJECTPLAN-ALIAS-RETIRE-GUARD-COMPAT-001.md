# 296x-993 LOCALFIRSTOBJECTPLAN-ALIAS-RETIRE-GUARD-COMPAT-001

Status: Landed
Date: 2026-06-17
Scope: guard compatibility / no alias removal

## Contract

```text
output_contract=hako-localfirstobjectplan-alias-retire-guard-compat-v0
source_evidence=296x-828,296x-991,296x-992
row_kind=guard_compat
historical_guards_tolerate_alias_retire=1
alias_present_or_retired_marker_required=1
objectplan_canonical_name_required=1
public_alias_currently_enabled=1
alias_removed=0
report_field_changed=0
vocabulary_merge_count=0
backend_lowering_changed=0
mirbuilder_object_management_enabled=0
smallest_safe_next=LOCALFIRSTOBJECTPLAN-ALIAS-RETIRE-IMPLEMENTATION-001
summary=ok
```

## Purpose

Make historical guards compatible with a future `LocalFirstObjectPlan` alias
retirement while preserving current behavior.

The alias is still present. This row only updates guard expectations so the next
row can remove the alias by adding an explicit retired marker instead of
breaking historical phase guards.

## Guard Compatibility Rule

```text
ObjectPlan is the canonical vocabulary.

Historical guards that previously required:
  pub type LocalFirstObjectPlan = ObjectPlan

now accept:
  pub type LocalFirstObjectPlan = ObjectPlan
  OR
  ("local_first_object_plan_alias_retired", "1")
```

The retired marker is not introduced in this row. It is reserved for the actual
alias removal row.

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

## Verification

```bash
bash tools/checks/k2_wide_phase296x_localfirstobjectplan_alias_retire_guard_compat_guard.sh
bash tools/checks/k2_wide_phase296x_object_plan_local_first_guard.sh
bash tools/checks/k2_wide_phase296x_objectplan_passive_unify_guard.sh
bash tools/checks/k2_wide_phase296x_routeplan_objectplan_handoff_guard.sh
bash tools/checks/k2_wide_phase296x_localfirstobjectplan_alias_retire_preflight_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
