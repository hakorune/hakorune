# 296x-1404 VARIABLE-CONTEXT-CARRIER-SNAPSHOT-PLAN-PROBE-001

Status: closed
Date: 2026-06-20

## Purpose

Add focused lifecycle fixtures for `CarrierInfo::from_variable_map` as a
snapshot from an owner-carrying read `BorrowView`.

## Selected By

```text
296x-1403-POST-CARRIER-PHI-INVENTORY-OWNER-SELECTION-001
```

## Scope

```text
source=src/mir/join_ir/lowering/carrier_info/carrier_info_impl.rs
method=CarrierInfo::from_variable_map
plan_kind=CarrierSnapshotFromBorrowView
```

Allowed:

```text
facts/plan fixtures for automatic carrier snapshot
oracle vector for loop_var + non-loop carrier extraction
guard that validates owner-carrying BorrowView and deterministic order inputs
```

## Non-Goals

```text
do_not_model_with_explicit_carriers=1
do_not_model_join_id_lifecycle=1
do_not_model_promoted_body_locals=1
do_not_model_trim_helper=1
do_not_add_general_resolver=1
do_not_claim_full_VariableContext_parity=1
```

## Acceptance

```text
carrier_snapshot_facts_fixture=green
carrier_snapshot_plan_fixture=green
carrier_snapshot_oracle_vectors=green
requires_owner_carrying_BorrowView=1
requires_deterministic_order=1
requires_ValueId_TrivialMemory=1
PHI_join_id_claim=0
general_resolver_implemented=0
```

Checks:

```bash
bash tools/checks/rust_lifecycle_variable_context_carrier_snapshot_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

## Closeout

```text
carrier_snapshot_facts_fixture=green
carrier_snapshot_plan_fixture=green
carrier_snapshot_oracle_vectors=green
requires_owner_carrying_BorrowView=1
requires_deterministic_order=1
requires_ValueId_TrivialMemory=1
PHI_join_id_claim=0
general_resolver_implemented=0
```

Evidence:

```bash
bash tools/checks/rust_lifecycle_variable_context_carrier_snapshot_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

Guard output:

```text
output_contract=rust-lifecycle-variable-context-carrier-snapshot-v0
carrier_snapshot_facts_fixture=green
carrier_snapshot_plan_fixture=green
carrier_snapshot_oracle_vectors=green
requires_owner_carrying_BorrowView=green
requires_deterministic_order=green
requires_ValueId_TrivialMemory=green
mutates_VariableContext=0
publishes_variable_map=0
PHI_join_id_claim=0
general_resolver_implemented=0
summary=ok
```

Next:

```text
296x-1405-POST-CARRIER-SNAPSHOT-OWNER-SELECTION-001
```

## Stop Line

```text
do_not_treat_carrier_snapshot_as_PHI_lifecycle=1
do_not_mutate_VariableContext=1
do_not_publish_variable_map=1
do_not_start_resolver_from_this_row=1
```
