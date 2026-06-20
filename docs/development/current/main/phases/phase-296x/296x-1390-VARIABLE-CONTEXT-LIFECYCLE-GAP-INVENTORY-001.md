# 296x-1390 VARIABLE-CONTEXT-LIFECYCLE-GAP-INVENTORY-001

Status: open
Date: 2026-06-20

## Purpose

Inventory the lifecycle gaps in MirBuilder `VariableContext` before creating a
facts/plan pilot.

This row is selection/inventory only. It must not implement a VariableContext
lifecycle projection.

## Selected By

```text
296x-1389-RUST-LIFECYCLE-NEXT-OWNER-SELECTION-001
```

## Scope

Inventory these VariableContext lifecycle shapes:

```text
BTreeMap<String, ValueId> deterministic iteration
&self read methods
&mut self mutation methods
returned &BTreeMap from variable_map()
returned &mut BTreeMap from variable_map_mut()
snapshot clone policy
restore ownership transfer
SSA renaming overwrite
PHI/carrier-sensitive map use
memory-only Drop erase preconditions
```

Expected output:

```text
accepted_for_next_pilot:
  simple map ownership, read, mutation, deterministic iteration

deny_or_redesign_before_pilot:
  returned mutable map borrow
  snapshot/restore clone ownership
  carrier-sensitive external map consumers
```

## Acceptance

```text
variable_context_lifecycle_gap_inventory_exists=1
returned_mutable_map_gap_identified=1
snapshot_restore_gap_identified=1
carrier_consumer_gap_identified=1
next_variable_context_slice_selected=1
implementation_started=0
facts_plan_pilot_started=0
general_resolver_implemented=0
```

Checks:

```bash
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

## Stop Line

```text
do_not_create_VariableContext_facts_plan_fixture_in_this_row=1
do_not_change_VariableContext_Rust_or_Hako_code=1
do_not_implement_lifecycle_resolver=1
do_not_claim_VariableContext_lifecycle_parity=1
```
