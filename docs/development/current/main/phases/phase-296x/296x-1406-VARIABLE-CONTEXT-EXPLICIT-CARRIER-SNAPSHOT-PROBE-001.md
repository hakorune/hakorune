# 296x-1406 VARIABLE-CONTEXT-EXPLICIT-CARRIER-SNAPSHOT-PROBE-001

Status: closed
Date: 2026-06-20

## Purpose

Add focused lifecycle fixtures for `CarrierInfo::with_explicit_carriers` as a
snapshot from an owner-carrying read `BorrowView` with explicit requested
carrier names.

## Selected By

```text
296x-1405-POST-CARRIER-SNAPSHOT-OWNER-SELECTION-001
```

## Scope

```text
source=src/mir/join_ir/lowering/carrier_info/carrier_info_impl.rs
method=CarrierInfo::with_explicit_carriers
plan_kind=ExplicitCarrierSnapshotFromBorrowView
```

Allowed:

```text
facts/plan fixtures for explicit carrier snapshot
oracle vector for requested carrier extraction
guard that validates requested-name ownership and missing-carrier fail-fast
```

## Non-Goals

```text
do_not_model_join_id_lifecycle=1
do_not_model_promoted_body_locals=1
do_not_model_trim_helper=1
do_not_add_general_resolver=1
do_not_claim_full_VariableContext_parity=1
```

## Acceptance

```text
explicit_carrier_snapshot_facts_fixture=green
explicit_carrier_snapshot_plan_fixture=green
explicit_carrier_snapshot_oracle_vectors=green
requires_owner_carrying_BorrowView=1
requires_requested_names_owned=1
missing_carrier_fail_fast_preserved=1
PHI_join_id_claim=0
general_resolver_implemented=0
```

Checks:

```bash
bash tools/checks/rust_lifecycle_variable_context_explicit_carrier_snapshot_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

## Closeout

```text
explicit_carrier_snapshot_facts_fixture=green
explicit_carrier_snapshot_plan_fixture=green
explicit_carrier_snapshot_oracle_vectors=green
requires_owner_carrying_BorrowView=1
requires_requested_names_owned=1
missing_carrier_fail_fast_preserved=1
PHI_join_id_claim=0
general_resolver_implemented=0
```

Evidence:

```bash
bash tools/checks/rust_lifecycle_variable_context_explicit_carrier_snapshot_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

Guard output:

```text
output_contract=rust-lifecycle-variable-context-explicit-carrier-snapshot-v0
explicit_carrier_snapshot_facts_fixture=green
explicit_carrier_snapshot_plan_fixture=green
explicit_carrier_snapshot_oracle_vectors=green
requires_owner_carrying_BorrowView=green
requires_requested_names_owned=green
missing_carrier_fail_fast_preserved=green
mutates_VariableContext=0
publishes_variable_map=0
PHI_join_id_claim=0
general_resolver_implemented=0
summary=ok
```

Next:

```text
296x-1407-POST-EXPLICIT-CARRIER-SNAPSHOT-OWNER-SELECTION-001
```

## Stop Line

```text
do_not_treat_requested_names_as_borrowed_aliases=1
do_not_silently_drop_missing_carriers=1
do_not_treat_explicit_carrier_snapshot_as_PHI_lifecycle=1
do_not_start_resolver_from_this_row=1
```
