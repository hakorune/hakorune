---
Status: Landed
Date: 2026-06-15
Task: COREPLAN-VARMAP-RESEAL-002
Scope: Move generic_loop_body variable_map publication behind var_map_scope helpers.
Related:
  - docs/development/current/main/workstreams/compiler-foundation-current.md
  - docs/development/current/main/phases/phase-293x/293x-1035-COREPLAN-NEXT-ROW-SELECTION-001.md
  - docs/development/current/main/phases/phase-293x/293x-1034-COREPLAN-VARMAP-RESEAL-001-PARTS-STMT.md
  - docs/development/current/main/phases/phase-293x/293x-1022-COREPLAN-VARMAP-BOUNDARY-001.md
  - src/mir/builder/control_flow/plan/parts/var_map_scope.rs
  - src/mir/builder/control_flow/plan/features/generic_loop_body/v0.rs
  - src/mir/builder/control_flow/plan/features/generic_loop_body/helpers.rs
  - src/mir/builder/control_flow/plan/features/generic_loop_body/carriers.rs
  - tools/checks/coreplan_varmap_boundary_inventory_guard.sh
---

# COREPLAN-VARMAP-RESEAL-002: generic_loop_body Reseal Helper

## Decision

This is a BoxShape-only follow-up to `COREPLAN-VARMAP-RESEAL-001`.

```text
selected_row=generic_loop_body_varmap_reseal
accepted_shape_added=0
fallback_route_added=0
release_default_changed=0
```

`branch_bindings` / carrier state remain the logical CorePlan path state.
`builder.variable_map` remains a defined-value emission cache. The
`generic_loop_body` family now publishes already-defined values through
`var_map_scope::publish_emission_cache` instead of writing to the map directly.

## Implementation

```text
parts/var_map_scope:
  publish_emission_cache visibility widened to control_flow::plan only

features/generic_loop_body/v0:
  assignment/local init publication uses publish_emission_cache

features/generic_loop_body/helpers:
  effect-only assignment/local init publication uses publish_emission_cache

features/generic_loop_body/carriers:
  loop carrier and loop-var current value publication uses publish_emission_cache

guard:
  variable_map_direct_insert_sites=48
  generic_loop_body_direct_variable_map_insert_sites=0
```

## Acceptance

```text
coreplan_varmap_reseal_generic_loop_body=1
generic_loop_body_direct_variable_map_insert_sites=0
variable_map_direct_insert_sites=48
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
do not make variable_map early PHI truth
do not combine this with PHI lifecycle transaction changes
do not rewrite all remaining direct variable_map sites in one row
do not widen var_map_scope outside control_flow::plan
```

## Next

```text
COREPLAN-PHI-TXN-001:
  define a PhiTxn-style lifecycle wrapper around Reserve / Define / Populate /
  Finalize before migrating broader PHI construction paths.
```
