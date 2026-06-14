---
Status: Landed
Date: 2026-06-14
Task: COREPLAN-VARMAP-BOUNDARY-001
Scope: BoxShape inventory/no-growth guard for `variable_map` write boundary.
Related:
  - docs/development/current/main/workstreams/compiler-foundation-current.md
  - docs/development/current/main/design/local-patch-prevention-ssot.md
  - docs/development/current/main/phases/phase-293x/293x-1021-COREPLAN-PHI-BINDING-SSOT-001.md
  - tools/checks/coreplan_varmap_boundary_inventory_guard.sh
---

# COREPLAN-VARMAP-BOUNDARY-001

## Decision

This is a BoxShape sidecar before `COREPLAN-PORT07-TIMEOUT-001`.

The goal is to prevent another local patch chain around logical bindings.
`current_bindings` / BindingState remains the logical CorePlan truth.
`variable_map` is a defined-value emission cache and compatibility lookup
surface. It must not become early PHI truth or recipe verification truth.

## Inventory

The robust inventory counts multiline `variable_map . insert(...)` forms under:

```text
src/mir/builder/control_flow/plan/**
src/mir/builder/ssa/**
```

Current count:

```text
variable_map_direct_insert_sites=62
variable_map_remove_clear_sites=0
```

Family split:

| family | count | reading |
| --- | ---: | --- |
| `features` | 16 | route-local reseal / route-local state publication |
| `generic_loop_body` | 11 | legacy generic-loop body lowering and carrier reseal |
| `normalizer` | 12 | expression/prelude lowering compatibility writes |
| `parts` | 9 | common stmt / conditional update / var-map scope reseal |
| `parts_dispatch` | 5 | dispatch-level join/block state publication |
| `parts_loop` | 4 | loop final-value and loop-v0 state publication |
| `composer` | 4 | legacy nested-minimal composer compatibility writes |
| `lowerer` | 1 | loop completion final publication |

## Contract

```text
logical_binding_truth_owner=current_bindings
variable_map_role=defined_value_emission_cache
variable_map_direct_insert_sites=62
variable_map_remove_clear_sites=0
variable_map_no_growth_guard=1
accepted_shape_added=0
fallback_route_added=0
```

## Proof

```bash
bash tools/checks/coreplan_varmap_boundary_inventory_guard.sh
```

## Next

Do not rewrite all sites in one pass. Choose one family at a time:

```text
1. parts/final_values or parts/stmt:
   small common owner candidate for explicit reseal helper.

2. normalizer:
   keep expression lowering compatibility separate from CorePlan logical truth.

3. generic_loop_body:
   legacy-route cleanup only after PORT07 owner is understood.
```

## Stop Line

```text
do not add new direct variable_map writes under CorePlan/SSA
do not use variable_map as early PHI truth
do not let LocalSSA repair logical binding freshness
do not combine this boundary cleanup with a new accepted CorePlan shape
```

