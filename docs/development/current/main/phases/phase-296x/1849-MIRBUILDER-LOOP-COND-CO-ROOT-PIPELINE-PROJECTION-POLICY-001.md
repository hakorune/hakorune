# 1849 - MIRBUILDER-LOOP-COND-CO-ROOT-PIPELINE-PROJECTION-POLICY-001

## Token

```text
MIRBUILDER-LOOP-COND-CO-ROOT-PIPELINE-PROJECTION-POLICY-001
```

## Purpose

Resolve whether `lower_loop_cond_continue_only` should become a standalone
Hako projection surface.

## Decision

```text
selected_policy = RootPipelineIntegrationOwner
owner_edge = loop_cond_continue_only
projection_surface_selected = 0
```

The source surface is the root loop-cond continue-only pipeline. It collects
carrier variables, builds carrier initial values, creates the core loop frame,
lowers the header condition, dispatches body lowering, applies fallthrough
continue cleanup, materializes/verifies PHI closure, and emits the final
`CorePlan::Loop`.

That makes it an integration/root pipeline owner for the
`loop_cond_continue_only` owner edge, not a leaf semantic projection surface.

## Output

```text
fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-loop-cond-co-root-pipeline-projection-policy-v0.json

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_loop_cond_co_root_pipeline_projection_policy_guard.sh
```

## Evidence

```text
visibility:
  pub(in crate::mir::builder)

callsites:
  src/mir/builder/control_flow/plan/recipe_tree/loop_cond_composer.rs
  src/mir/builder/control_flow/plan/normalizer/mod.rs

return_type:
  Result<LoweredRecipe, String>

root pipeline markers:
  carriers::collect_from_recipe_continue_only
  collect_carrier_inits
  build_coreloop_frame
  lower_loop_header_cond
  lower_continue_only_block
  apply_fallthrough_continue_exit
  materialize_loop_cond_continue_only_phi_closure
  verify_loop_cond_continue_only_phi_closure
  CorePlan::Loop
```

## Acceptance

```text
policy = RootPipelineIntegrationOwner
decision = KeepParentOwner
projection_surface_selected = 0
manual_family_selection = 0
hako_generation = 0
hako_adopted_decision = 0
native_seed_materialization = 0
source_selfhost_claim = 0
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
new_python_semantic_projector = 0
runner_semantic_owner = 0
```

## Recommended Next Tasks

```text
1. MIRBUILDER-LOOP-COND-CO-BLOCK-LOWERING-PROJECTION-POLICY-001
```

## Non-Claims

```text
no standalone Hako projection surface
no Hako generation
no HakoAdopted decision
no native seed materialization
no Source Selfhost claim
no route repair
```
