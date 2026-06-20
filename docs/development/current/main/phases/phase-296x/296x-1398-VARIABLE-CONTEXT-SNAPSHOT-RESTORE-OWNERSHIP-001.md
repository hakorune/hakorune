# 296x-1398 VARIABLE-CONTEXT-SNAPSHOT-RESTORE-OWNERSHIP-001

Status: closed
Date: 2026-06-20

## Purpose

Add focused lifecycle facts/plan fixtures for `VariableContext::snapshot()` and
`VariableContext::restore()` ownership transfer.

## Selected By

```text
296x-1397-VARIABLE-CONTEXT-POST-BORROWVIEW-OWNER-SELECTION-001
```

## Scope

```text
snapshot():
  owned BTreeMap<String, ValueId> clone
  deterministic order preserved

restore(snapshot):
  ReplaceOwned transfer into VariableContext.variable_map
  previous map cleanup allowed only with TrivialMemory fact
```

Allowed:

```text
facts/plan fixtures for snapshot/restore
oracle vectors for clone/restore behavior
guard that validates deterministic-order and cleanup requirements
```

## Non-Goals

```text
do_not_model_variable_map_mut=1
do_not_model_carrier_PHI=1
do_not_add_general_resolver=1
do_not_change_Rust_API=1
do_not_claim_full_VariableContext_parity=1
```

## Acceptance

```text
snapshot_restore_facts_fixture=green
snapshot_restore_plan_fixture=green
snapshot_restore_oracle_vectors=green
snapshot_clone_requires_deterministic_order=1
restore_requires_ReplaceOwned=1
old_map_cleanup_requires_TrivialMemory=1
mutable_map_claim=0
carrier_PHI_claim=0
full_VariableContext_parity_claim=0
```

Checks:

```bash
bash tools/checks/rust_lifecycle_variable_context_snapshot_restore_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

## Closeout

```text
snapshot_restore_facts_fixture=green
snapshot_restore_plan_fixture=green
snapshot_restore_oracle_vectors=green
snapshot_clone_requires_deterministic_order=1
restore_requires_ReplaceOwned=1
old_map_cleanup_requires_TrivialMemory=1
mutable_map_claim=0
carrier_PHI_claim=0
full_VariableContext_parity_claim=0
```

Evidence:

```bash
bash tools/checks/rust_lifecycle_variable_context_snapshot_restore_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

Guard output:

```text
output_contract=rust-lifecycle-variable-context-snapshot-restore-v0
snapshot_restore_facts_fixture=green
snapshot_restore_plan_fixture=green
snapshot_restore_oracle_vectors=green
snapshot_clone_requires_deterministic_order=green
restore_requires_ReplaceOwned=green
old_map_cleanup_requires_TrivialMemory=green
mutable_map_claim=0
carrier_PHI_claim=0
full_VariableContext_parity_claim=0
summary=ok
```

Next:

```text
296x-1399-VARIABLE-CONTEXT-POST-SNAPSHOT-RESTORE-OWNER-SELECTION-001
```

## Stop Line

```text
do_not_treat_restore_as_direct_mutation_without_ownership_transfer=1
do_not_clone_map_without_deterministic_order_fact=1
do_not_erase_old_map_cleanup_without_TrivialMemory=1
do_not_fold_carrier_info_into_this_row=1
```
