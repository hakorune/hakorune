# 1836 - MIRBUILDER-LOOP-COND-BC-RETURN-ONLY-ELSE-PATTERN-PROJECTION-POLICY-001

## Token

```text
MIRBUILDER-LOOP-COND-BC-RETURN-ONLY-ELSE-PATTERN-PROJECTION-POLICY-001
```

## Purpose

Resolve whether the return-only loop-cond else-pattern lowering surfaces should
be standalone Hako projection surfaces.

## Decision

```text
selected_policy = PrivateLoweringHelper
owner_edge = loop_cond_break_continue
projection_surface_selected = 0
```

The two source surfaces are internal lowering helpers under
`loop_cond_break_continue`; they are not standalone semantic owner edges.

```text
lower_else_only_return_if
lower_then_only_return_if
```

## Output

```text
fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-loop-cond-bc-return-only-else-pattern-projection-policy-v0.json

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_loop_cond_bc_return_only_else_pattern_projection_policy_guard.sh
```

## Evidence

```text
visibility:
  pub(in crate::mir::builder::control_flow::plan::features)

callsite:
  src/mir/builder/control_flow/plan/features/loop_cond_bc_item.rs

return_type:
  Result<Vec<LoweredRecipe>, String>
```

## Acceptance

```text
policy = PrivateLoweringHelper
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
1. MIRBUILDER-LOOP-COND-BC-CONTINUE-IF-ELSE-PATTERN-PROJECTION-POLICY-001
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
