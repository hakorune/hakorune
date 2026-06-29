# 1846 - MIRBUILDER-LOOP-COND-BC-CARRIER-SYNC-PROJECTION-POLICY-001

## Token

```text
MIRBUILDER-LOOP-COND-BC-CARRIER-SYNC-PROJECTION-POLICY-001
```

## Purpose

Resolve whether `sync_carrier_bindings` should become a standalone Hako
projection surface.

## Decision

```text
selected_policy = PrivateCarrierSyncHelper
owner_edge = loop_cond_break_continue
projection_surface_selected = 0
```

The source surface fills missing carrier bindings after nested loop lowering.
It does not overwrite existing `current_bindings`; it only reads the current
`builder.variable_ctx.variable_map` value when the carrier binding is missing.

This is a route-local carrier synchronization helper under
`loop_cond_break_continue`, not a standalone semantic owner edge.

## Output

```text
fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-loop-cond-bc-carrier-sync-projection-policy-v0.json

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_loop_cond_bc_carrier_sync_projection_policy_guard.sh
```

## Evidence

```text
visibility:
  pub(super)

callsites:
  src/mir/builder/control_flow/plan/features/loop_cond_bc_item.rs
  src/mir/builder/control_flow/plan/features/loop_cond_bc_item_stmt.rs

return_type:
  <none>

carrier sync markers:
  Do not overwrite existing carrier bindings
  current_bindings.contains_key
  builder.variable_ctx.variable_map.get
  current_bindings.insert
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
1. MIRBUILDER-LOOP-COND-BC-NESTED-CARRIER-PROJECTION-POLICY-001
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
