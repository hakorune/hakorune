# 1860 - MIRBUILDER-LOOP-COND-CO-HELPER-MUTATION-PROBE-PROJECTION-POLICY-001

## Token

```text
MIRBUILDER-LOOP-COND-CO-HELPER-MUTATION-PROBE-PROJECTION-POLICY-001
```

## Purpose

Resolve whether `map_mutates_existing_vars` should become a standalone Hako
projection surface.

## Decision

```text
selected_policy = PrivateMutationProbeHelper
owner_edge = loop_cond_continue_only
projection_surface_selected = 0
```

The source surface is a route-local predicate used to reject group-if
fallthrough mutation that would require join generation. It compares ValueId
bindings for names already present before the branch.

It is not a standalone semantic owner edge.

## Output

```text
fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-loop-cond-co-helper-mutation-probe-projection-policy-v0.json

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_loop_cond_co_helper_mutation_probe_projection_policy_guard.sh
```

## Evidence

```text
visibility:
  pub(super)

callsite:
  src/mir/builder/control_flow/plan/features/loop_cond_co_group_if.rs

return_type:
  bool

mutation-probe markers:
  for (name, pre_id) in pre
  if let Some(post_id) = post.get(name)
  if post_id != pre_id
  return true
  false
```

## Acceptance

```text
policy = PrivateMutationProbeHelper
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
1. MIRBUILDER-LOOP-COND-CO-HELPER-CARRIER-SYNC-PROJECTION-POLICY-001
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
