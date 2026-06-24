# 296x-1396 VARIABLE-CONTEXT-IMMUTABLE-MAP-BORROWVIEW-PROBE-001

Status: closed
Date: 2026-06-20

## Purpose

Add a focused, read-only lifecycle probe for
`VariableContext::variable_map()` as an owner-carrying `BorrowView` candidate.

## Selected By

```text
296x-1395-VARIABLE-CONTEXT-POST-RETURNED-BORROW-OWNER-SELECTION-001
```

## Scope

```text
method=variable_map()
borrow_kind=SharedRead
candidate_plan=BorrowView(read)
consumer_scope=tests_and_region_observation_only
```

Allowed:

```text
facts/plan fixtures for immutable returned map borrow
guard that validates owner-carrying BorrowView constraints
oracle vectors for read-only key/entry observation if needed
```

## Non-Goals

```text
do_not_model_variable_map_mut=1
do_not_model_snapshot_restore=1
do_not_model_carrier_PHI=1
do_not_add_general_resolver=1
do_not_emit_naked_borrow_alias=1
do_not_claim_full_VariableContext_parity=1
```

## Acceptance

```text
immutable_map_borrow_facts_fixture=green
immutable_map_borrow_plan_fixture=green
owner_carrying_borrowview_required=1
borrow_escape_denied=1
carrier_PHI_claim=0
mutable_map_claim=0
snapshot_restore_claim=0
implementation_scope_limited_to_fixtures_and_guard=1
```

Checks:

```bash
bash tools/checks/rust_lifecycle_variable_context_immutable_borrow_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

## Closeout

```text
immutable_map_borrow_facts_fixture=green
immutable_map_borrow_plan_fixture=green
owner_carrying_borrowview_required=1
borrow_escape_denied=1
carrier_PHI_claim=0
mutable_map_claim=0
snapshot_restore_claim=0
implementation_scope_limited_to_fixtures_and_guard=1
```

Evidence:

```bash
bash tools/checks/rust_lifecycle_variable_context_immutable_borrow_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

Guard output:

```text
output_contract=rust-lifecycle-variable-context-immutable-borrow-v0
immutable_map_borrow_facts_fixture=green
immutable_map_borrow_plan_fixture=green
immutable_map_borrow_oracle_vectors=green
owner_carrying_borrowview_required=green
borrow_escape_denied=green
mutable_map_claim=0
snapshot_restore_claim=0
carrier_PHI_claim=0
full_VariableContext_parity_claim=0
summary=ok
```

Next:

```text
296x-1397-VARIABLE-CONTEXT-POST-BORROWVIEW-OWNER-SELECTION-001
```

## Stop Line

```text
do_not_treat_shared_borrow_as_copied_map=1
do_not_treat_shared_borrow_as_ordered_map_owner=1
do_not_allow_escaping_naked_alias=1
do_not_fold_carrier_info_into_this_probe=1
```
