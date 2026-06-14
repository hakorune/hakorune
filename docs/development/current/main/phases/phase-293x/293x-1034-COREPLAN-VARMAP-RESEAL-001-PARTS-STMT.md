---
Status: Landed
Date: 2026-06-14
Task: COREPLAN-VARMAP-RESEAL-001
Scope: Move selected `parts/**` variable_map publication behind var_map_scope helpers.
Related:
  - docs/development/current/main/workstreams/compiler-foundation-current.md
  - docs/development/current/main/phases/phase-293x/293x-1022-COREPLAN-VARMAP-BOUNDARY-001.md
  - src/mir/builder/control_flow/plan/parts/stmt.rs
  - src/mir/builder/control_flow/plan/parts/var_map_scope.rs
  - tools/checks/coreplan_varmap_boundary_inventory_guard.sh
---

# COREPLAN-VARMAP-RESEAL-001: selected parts Reseal Helper

## Decision

This is a BoxShape-only follow-up to `COREPLAN-ONE-ROW-IMPL-001`.

```text
selected_row=parts_stmt_varmap_reseal
accepted_shape_added=0
fallback_route_added=0
release_default_changed=0
```

`branch_bindings` remains the logical CorePlan path state.
`builder.variable_map` remains a defined-value emission cache. Selected
`parts/**` owners now publish through `var_map_scope` helpers instead of
writing directly.

## Implementation

```text
var_map_scope:
  publish_defined_binding(...)
  reseal_branch_bindings(...)

parts/stmt:
  direct variable_map insert sites removed
  branch_bindings publication order preserved

parts/conditional_update:
  direct variable_map insert site removed
  conditional current_bindings update policy preserved

parts/loop_/final_values:
  direct variable_map insert site removed
  current_bindings update policy preserved

guard:
  variable_map_direct_insert_sites=54
  parts_stmt_direct_variable_map_insert_sites=0
  selected_parts_direct_variable_map_insert_sites=0
```

## Acceptance

```text
coreplan_varmap_reseal_parts_stmt=1
parts_stmt_direct_variable_map_insert_sites=0
selected_parts_direct_variable_map_insert_sites=0
variable_map_direct_insert_sites=54
variable_map_role=defined_value_emission_cache
current_bindings_truth_owner_preserved=1
accepted_shape_added=0
fallback_route_added=0
release_default_changed=0
summary=ok
```

## Proof Commands

```bash
bash tools/checks/coreplan_varmap_boundary_inventory_guard.sh
bash tools/checks/coreplan_phi_binding_boundary_guard.sh
cargo check --bin hakorune
```

## Stop Line

```text
do not make variable_map logical binding truth
do not combine this with PHI lifecycle changes
do not rewrite all variable_map sites in one pass
do not lower the guard baseline without reducing direct write sites
```
