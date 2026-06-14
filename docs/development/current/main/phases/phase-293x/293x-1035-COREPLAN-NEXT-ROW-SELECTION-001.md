---
Status: Active
Date: 2026-06-15
Task: COREPLAN-NEXT-ROW-SELECTION-001
Scope: Select the next one-purpose CorePlan / JoinIR compiler-foundation row after COREPLAN-VARMAP-RESEAL-001.
Related:
  - docs/development/current/main/workstreams/compiler-foundation-current.md
  - docs/development/current/main/phases/phase-293x/293x-1034-COREPLAN-VARMAP-RESEAL-001-PARTS-STMT.md
  - docs/development/current/main/phases/phase-293x/293x-1021-COREPLAN-PHI-BINDING-SSOT-001.md
  - docs/development/current/main/phases/phase-293x/293x-1022-COREPLAN-VARMAP-BOUNDARY-001.md
  - docs/development/current/main/design/phi-lifecycle-ssot.md
  - docs/development/current/main/design/coreplan-compat-normalizer-legoization-ssot.md
  - docs/development/current/main/workstreams/arc-retirement-current.md
---

# COREPLAN-NEXT-ROW-SELECTION-001

## Decision

Compiler foundation remains the active lane. The next action is a planning-only
selection row.

```text
selected_mode=compiler_first
implementation_started=0
boxcount_boxshape_mixed=0
exact_front_optimization_resumed=0
arc_global_replacement_started=0
```

Arc retirement is not the next active implementation row. Its first-family
host-handle text payload cutover is already tracked in the side-lane taskboard,
and global object substrate replacement remains closed until compiler
foundation reaches an explicit pause or closeout.

Mimalloc optimization also remains paused. It resumes later through
`MIMALLOC-AOT-KERNEL-FRONT-SELECT-002`, not through a compiler-foundation row.

## Candidate Rows

### Candidate A: COREPLAN-VARMAP-RESEAL-002

Owner:

```text
var_map_scope / CorePlan binding boundary
```

Scope:

```text
reduce one remaining variable_map direct-write family below the 54-site
baseline
```

Likely first family:

```text
generic_loop_body or normalizer
```

Acceptance:

```text
variable_map_direct_insert_sites_decreased=1
current_bindings_truth_owner_preserved=1
accepted_shape_added=0
fallback_route_added=0
release_default_changed=0
```

Stop line:

```text
do not make variable_map logical binding truth
do not make variable_map early PHI truth
do not combine with PHI lifecycle migration
do not rewrite all remaining direct-write sites in one row
```

### Candidate B: COREPLAN-PHI-TXN-001

Owner:

```text
phi_lifecycle
```

Scope:

```text
define a PhiTxn-style lifecycle wrapper around Reserve / Define / Populate /
Finalize so provisional PHI paths cannot bypass lifecycle ordering
```

Acceptance:

```text
phi_transaction_boundary_defined=1
low_level_phi_lifecycle_bypass_count=0
accepted_shape_added=0
fallback_route_added=0
release_default_changed=0
```

Stop line:

```text
do not migrate JoinIR merge PHI builders in the same row
do not add fallback for missing current block
do not change accepted source shapes
do not make variable_map PHI truth
```

### Candidate C: COREPLAN-JOINIR-MERGE-PHI-001

Owner:

```text
JoinIR merge builder boundary + phi_lifecycle
```

Scope:

```text
migrate exactly one JoinIR merge PHI construction path to the PHI lifecycle API
```

Acceptance:

```text
one_joinir_merge_phi_path_migrated=1
phi_lifecycle_owner_preserved=1
accepted_shape_added=0
fallback_route_added=0
release_default_changed=0
```

Stop line:

```text
one merge path only
do not broaden into all PHI constructors
do not route PHI through TypeAbiCatalog or BoxCallableRegistry
do not expand variable_map truth
```

### Candidate D: COREPLAN-NORMALIZER-COMPOSITION-001

Owner:

```text
CorePlan normalizer / Recipe composition boundary
```

Scope:

```text
move one remaining AST-owned normalizer decision behind a named adapter or
Recipe / VerifiedRecipe boundary
```

Acceptance:

```text
one_normalizer_ast_decision_moved_to_adapter=1
normalizer_composition_only_progress=1
accepted_shape_added=0
fallback_route_added=0
release_default_changed=0
```

Stop line:

```text
do not move AST matching sideways without an adapter SSOT
do not add accepted shapes
do not mix with variable_map reseal or PHI lifecycle work
```

## Recommended Selection

Recommended next implementation row:

```text
COREPLAN-VARMAP-RESEAL-002
```

Reason:

```text
COREPLAN-VARMAP-RESEAL-001 already reduced selected parts/** publishers
behind var_map_scope helpers. Continuing with exactly one additional direct
variable_map write family keeps the compiler-first lane BoxShape-only and
reduces future local-patch risk before opening PHI transaction or JoinIR merge
migration.
```

Second choice:

```text
COREPLAN-PHI-TXN-001
```

Use it instead only if the next failing gate points at PHI lifecycle order
rather than variable_map publication spread.

## Non-Goals

```text
do not start Arc global replacement
do not start VMValue::BoxRef carrier migration
do not resume mimalloc exact-front optimization
do not add source-level concurrency syntax
do not add .hako workaround for compiler expressivity blockers
do not combine CorePlan BoxShape cleanup with BoxCount acceptance expansion
```

## Proof / Check Commands

Planning-only row:

```bash
bash tools/checks/current_state_pointer_guard.sh
```

Next row, if `COREPLAN-VARMAP-RESEAL-002` is selected:

```bash
bash tools/checks/coreplan_varmap_boundary_inventory_guard.sh
bash tools/checks/coreplan_phi_binding_boundary_guard.sh
```

## Acceptance

```text
coreplan_next_row_selection_landed=1
next_compiler_row_selected=COREPLAN-VARMAP-RESEAL-002
arc_retirement_side_lane_preserved=1
arc_global_replacement_started=0
exact_front_optimization_resumed=0
mimalloc_return_lane=MIMALLOC-AOT-KERNEL-FRONT-SELECT-002
boxcount_boxshape_mixed=0
implementation_started=0
summary=ok
```
