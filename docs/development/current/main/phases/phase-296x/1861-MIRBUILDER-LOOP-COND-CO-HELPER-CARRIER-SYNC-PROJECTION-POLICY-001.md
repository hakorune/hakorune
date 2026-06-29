# 1861 - MIRBUILDER-LOOP-COND-CO-HELPER-CARRIER-SYNC-PROJECTION-POLICY-001

## Token

```text
MIRBUILDER-LOOP-COND-CO-HELPER-CARRIER-SYNC-PROJECTION-POLICY-001
```

## Purpose

Resolve whether `sync_carrier_bindings` should become a standalone Hako
projection surface.

## Decision

```text
selected_policy = PrivateCarrierSyncHelper
owner_edge = loop_cond_continue_only
projection_surface_selected = 0
```

The source surface is a route-local synchronization helper used after nested
loop lowering. It copies only carrier phi names from `variable_ctx` into
`current_bindings`.

It is not a standalone semantic owner edge.

## Output

```text
fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-loop-cond-co-helper-carrier-sync-projection-policy-v0.json

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_loop_cond_co_helper_carrier_sync_projection_policy_guard.sh
```

## Evidence

```text
visibility:
  pub(super)

callsite:
  src/mir/builder/control_flow/plan/features/loop_cond_co_stmt.rs

return_type:
  void

carrier-sync markers:
  for (name, _) in carrier_phis
  builder.variable_ctx.variable_map.get(name)
  current_bindings.insert(name.clone(), *value_id)
```

## Acceptance

```text
policy = PrivateCarrierSyncHelper
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
1. MIRBUILDER-LOOP-COND-CO-STATEMENT-LOWERING-SURFACE-CLASSIFICATION-001
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
