# 296x-1391 VARIABLE-CONTEXT-LIFECYCLE-SIMPLE-MAP-PILOT-001

Status: open
Date: 2026-06-20

## Purpose

Create a narrow VariableContext lifecycle facts/plan pilot for the simple
map-owned methods only.

## Selected By

```text
296x-1390-VARIABLE-CONTEXT-LIFECYCLE-GAP-INVENTORY-001
```

## Scope

Included:

```text
new/default
lookup
contains
len
is_empty
insert
remove
deterministic iteration expectation
SSA overwrite with TrivialMemory ValueId
memory-only Drop erase with TrivialMemory
```

Excluded:

```text
variable_map()
variable_map_mut()
snapshot()
restore()
carrier extraction consumers
PHI planner integration
```

## Acceptance

```text
variable_context_simple_map_facts_fixture=1
variable_context_simple_map_plan_fixture=1
returned_map_methods_excluded=1
snapshot_restore_excluded=1
carrier_consumers_excluded=1
ordered_map_projection_requires_deterministic_order_fact=1
memory_drop_erased_only_with_TrivialMemory=1
general_resolver_implemented=0
converter_emission_added=0
rust_lifetime_syntax_added=0
```

Checks:

```bash
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

## Stop Line

```text
do_not_model_variable_map_mut=1
do_not_model_snapshot_restore=1
do_not_claim_carrier_or_PHI_parity=1
do_not_implement_general_resolver=1
```
