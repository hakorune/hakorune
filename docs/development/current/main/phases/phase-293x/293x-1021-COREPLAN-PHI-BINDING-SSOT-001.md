---
Status: Landed
Date: 2026-06-14
Task: COREPLAN-PHI-BINDING-SSOT-001
Scope: BoxShape stop-the-line for PHI / binding / RecipeOnly fallback ownership.
Related:
  - docs/development/current/main/workstreams/compiler-foundation-current.md
  - docs/development/current/main/design/phi-lifecycle-ssot.md
  - docs/development/current/main/design/local-patch-prevention-ssot.md
  - docs/development/current/main/design/compiler-pipeline-ssot.md
  - src/mir/policies/loop_body_lowering_policy.rs
  - src/mir/builder/control_flow/plan/features/loop_cond_bc.rs
  - src/mir/builder/control_flow/plan/features/nested_loop_depth1_preheader.rs
  - src/mir/builder/ssa/local.rs
  - src/mir/builder/ssa/phi_input_materializer.rs
---

# COREPLAN-PHI-BINDING-SSOT-001

## Decision

This is a BoxShape row, not a BoxCount row.

The PORT04 patch chain exposed responsibility drift around PHI lifecycle,
logical bindings, LocalSSA, and loop recipe fallback. Before adding any new
CorePlan acceptance shape, restore the ownership boundaries.

## Ownership Contract

```text
PHI lifecycle:
  owns Reserve / Define / Populate entry points
  provisional PHI patching must use the same edge materialization policy as
  final PHI insertion

BindingState / current_bindings:
  owns CorePlan logical value truth during planning/lowering

variable_map:
  defined-value emission cache only
  not early PHI truth
  not recipe verification truth when current_bindings is available

LocalSSA:
  block-local operand materialization only
  not logical binding freshness repair

RecipeOnly:
  recipe items lower exactly once, in order
  route-level whole-body ExitAllowed fallback is forbidden
```

## First Slice

```text
1. Remove hidden generic value capture from nested_loop_depth1 preheader
   freshness. Preheader freshness may remap block ids and PHI predecessors; it
   must not allocate/copy arbitrary external values.

2. Remove route-level whole-body fallback from RecipeOnly loop-cond lowering.
   Whole-body ExitAllowed lowering is allowed only when facts selected
   BodyLoweringPolicy::ExitAllowed.

3. Keep the PORT04 timeout script metadata, but do not close this row until the
   full phase29bq fast gate passes.
```

## Acceptance

```text
phi_binding_responsibility_ssot_updated=1
coreplan_phi_binding_boundary_guard=PASS
local_patch_prevention_ssot_updated=1
nested_loop_preheader_hidden_value_capture=0
recipe_only_whole_body_fallback=0
phase29bq_joinir_port04_phi_exit_invariant_lock_vm=PASS
phase29bq_fast_gate_vm_advances_to_next_independent_blocker=1
next_independent_blocker=phase29bq_joinir_port07_expr_parity_seed_vm_timeout
loop_v0_route_added=0
fixture_expected_output_changed=0
fallback_route_added=0
accepted_shape_added=0
```

## Result

```text
landed=1
hidden_preheader_value_capture_removed=1
recipeonly_route_level_exitallowed_fallback_removed=1
coreplan_phi_binding_boundary_guard_landed=1
non_propagating_nested_loop_final_values_applied=1
then_else_only_break_fallthrough_state_restored=1
focused_port04_gate=PASS
focused_nested_loop_gate=PASS
focused_then_only_break_assign_gate=PASS
full_fast_gate_first_failure=phase29bq_joinir_port07_expr_parity_seed_vm_timeout
```

Proof:

```bash
bash tools/checks/coreplan_phi_binding_boundary_guard.sh
```

## Stop Line

```text
do not add a new loop route while this row is active
do not let preheader freshness allocate/copy arbitrary external values
do not let LocalSSA repair CorePlan logical binding freshness
do not make variable_map the early PHI truth
do not convert RecipeOnly failure into whole-body fallback
```
