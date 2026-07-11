# 296x-1412 CARRIER-INFO-MERGE-FROM-LIFECYCLE-PROBE-001

Status: closed
Date: 2026-06-20

## Purpose

Fixture-guard `CarrierInfo::merge_from` as an owned `CarrierInfo` mutation
boundary.

## Selected By

```text
296x-1411-POST-JOIN-ID-PRODUCER-INVENTORY-OWNER-SELECTION-001
```

## Scope

```text
source=src/mir/join_ir/lowering/carrier_info/carrier_info_impl.rs
method=CarrierInfo::merge_from
plan_kind=OwnedCarrierInfoMerge
```

Allowed:

```text
facts/plan fixtures for merge_from ownership
oracle vectors for carrier dedupe/sort and promoted_body_locals dedupe
guard that validates no join_id producer or resolver claim
```

## Non-Goals

```text
do_not_add_join_id_producer=1
do_not_resolve_join_id_vocabulary=1
do_not_add_general_resolver=1
do_not_add_converter_emission=1
do_not_claim_full_VariableContext_parity=1
do_not_claim_MirBuilder_wide_lifecycle_parity=1
```

## Acceptance

```text
carrier_info_merge_from_facts_fixture=green
carrier_info_merge_from_plan_fixture=green
carrier_info_merge_from_oracle_vectors=green
requires_owned_receiver=1
requires_readonly_other_borrow=1
deduplicate_by_name=1
sort_after_merge=1
join_id_producer=0
general_resolver_implemented=0
```

Checks:

```bash
bash tools/checks/rust_lifecycle_carrier_info_merge_from_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

## Closeout

```text
carrier_info_merge_from_facts_fixture=green
carrier_info_merge_from_plan_fixture=green
carrier_info_merge_from_oracle_vectors=green
requires_owned_receiver=1
requires_readonly_other_borrow=1
deduplicate_by_name=1
sort_after_merge=1
join_id_producer=0
general_resolver_implemented=0
```

Evidence:

```bash
bash tools/checks/rust_lifecycle_carrier_info_merge_from_guard.sh
```

Guard output:

```text
output_contract=rust-lifecycle-carrier-info-merge-from-v0
carrier_info_merge_from_facts_fixture=green
carrier_info_merge_from_plan_fixture=green
carrier_info_merge_from_oracle_vectors=green
requires_owned_receiver=green
requires_readonly_other_borrow=green
deduplicate_by_name=green
sort_after_merge=green
join_id_producer=0
general_resolver_implemented=0
summary=ok
```

Next:

```text
296x-1413-POST-MERGE-FROM-LIFECYCLE-OWNER-SELECTION-001
```

## Stop Line

```text
do_not_treat_merge_from_as_join_id_producer=1
do_not_start_resolver_from_this_row=1
do_not_mutate_Rust_code_in_this_row=1
```
