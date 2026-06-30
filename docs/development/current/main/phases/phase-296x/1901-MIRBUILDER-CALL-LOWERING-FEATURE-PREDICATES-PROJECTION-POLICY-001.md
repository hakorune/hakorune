# 1901 - MIRBUILDER-CALL-LOWERING-FEATURE-PREDICATES-PROJECTION-POLICY-001

## Token

```text
MIRBUILDER-CALL-LOWERING-FEATURE-PREDICATES-PROJECTION-POLICY-001
```

## Purpose

Resolve the `CallFeaturePredicates` subcluster selected by the CallLowering
subcluster decomposition.

The selected predicates are:

```text
is_unified_call_enabled() -> bool
is_pure_method(box_name, method) -> bool
contains_value_return(nodes) -> bool
```

These predicates do not share one projection owner. They mix a config gate, a
pure-method catalog predicate, and an AST traversal. This card decomposes them
into narrower feature predicate subclusters and selects the config gate first.

## Output

```text
tool:
  tools/rust_lifecycle/
    mirbuilder_call_lowering_feature_predicates_projection_policy.py

fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-call-lowering-feature-predicates-projection-policy-v0.json

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_call_lowering_feature_predicates_projection_policy_guard.sh
```

## Feature Subclusters

```text
UnifiedCallModeGate:
  is_unified_call_enabled

PureMethodCatalog:
  is_pure_method

ValueReturnAstScan:
  contains_value_return
```

## Decision

```text
kind = SelectFeaturePredicateSubcluster
selected_feature_subcluster = UnifiedCallModeGate

next_card =
  MIRBUILDER-CALL-LOWERING-UNIFIED-CALL-MODE-GATE-PROJECTION-POLICY-001
```

## Acceptance

```text
feature_subcluster_count = 3
whole_feature_predicate_projection = 0
projection_surface_selected = 0
registry_descriptor_selected = 0
ast_traversal_projection_selected = 0
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
no whole feature predicate projection owner
no registry descriptor materialization
no AST traversal projection
no Hako generation
no HakoAdopted decision
no Source Selfhost claim
```
