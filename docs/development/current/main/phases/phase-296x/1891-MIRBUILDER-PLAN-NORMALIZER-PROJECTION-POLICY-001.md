# 1891 - MIRBUILDER-PLAN-NORMALIZER-PROJECTION-POLICY-001

## Token

```text
MIRBUILDER-PLAN-NORMALIZER-PROJECTION-POLICY-001
```

## Purpose

Resolve the selected PlanNormalizer projection-policy cluster.

The selected surfaces are local PlanNormalizer helpers:

```text
create_phi_bindings(bindings: &[(&str, ValueId)]) -> BTreeMap<String, ValueId>
is_pure_value_expr(ast: &ASTNode) -> bool
```

`create_phi_bindings` builds a temporary PHI lookup map for normalizer use.
`is_pure_value_expr` is a normalizer-local predicate for already-supported
value-if and pure expression paths. These helpers remain inside the
PlanNormalizer parent owner and do not open standalone Hako projection surfaces.

## Output

```text
tool:
  tools/rust_lifecycle/
    mirbuilder_plan_normalizer_projection_policy.py

fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-plan-normalizer-projection-policy-v0.json

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_plan_normalizer_projection_policy_guard.sh
```

## Decision

```text
policy = KeepParentOwner
owner_edge = mirbuilder::plan_normalizer
projection_surface_selected = 0

next_card =
  MIRBUILDER-PROJECTION-POLICY-CLUSTER-PRIORITY-RESOLUTION-001
```

## Evidence

```text
source_count = 2

roles:
  phi_binding_map_helper = 1
  normalizer_local_purity_predicate = 1

markers:
  Create phi_bindings map from variable name-ValueId pairs
  phi_bindings are used to override variable_map lookups
  is_known_pure_method_call_for_value_if
  Stage-B/JsonFrag normalizer uses ternary value-if
  Selfhost FuncLowering uses ternary value-if
  ASTNode::BlockExpr
  BinaryOperator::Add
```

## Acceptance

```text
policy = KeepParentOwner
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

## Non-Claims

```text
no standalone Hako projection surface
no Hako generation
no HakoAdopted decision
no native seed materialization
no Source Selfhost claim
```
