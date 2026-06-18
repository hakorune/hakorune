Status: Done
Date: 2026-06-18
Scope: select next passive family after ArrayRecord bundle split
Related:
  - docs/development/current/main/design/build-crate-split-plan-ssot.md
  - src/mir/function/object_metadata.rs

# BUILD-MIR-PLANS-NEXT-PASSIVE-FAMILY-SELECTION-001

## Purpose

Select the next safe passive family after record-layout / ArrayRecord /
PackedArray metadata rows moved into `hakorune-mir-plans`.

## Decision

Select the object-state passive plan bundle:

```text
selected_family=object_state_passive_bundle
move=TypedObjectPlan,DirectStatePlan,RecordStateResidencePlan,RecordStateFieldAccessPlan
keep_main_crate=UserBoxFieldDecl,RecordDecl,typed object producers,direct state producers,record state producers
behavior_changed=0
```

`UserBoxFieldDecl` and `RecordDecl` stay in the main crate because they are
declaration/input inventory, not plan rows.

## Contract

```text
output_contract=build-mir-plans-next-passive-family-selection-v0

selected_family=object_state_passive_bundle
boxshape_only=1
boxcount_allowed=0
behavior_change_allowed=0
declaration_inventory_moved=0
producer_logic_moved=0
backend_lowering_enabled=0
runtime_route_enabled=0

summary=ok
```

## Next

```text
next_task=BUILD-MIR-PLANS-OBJECT-STATE-PASSIVE-BUNDLE-SPLIT-001
```
