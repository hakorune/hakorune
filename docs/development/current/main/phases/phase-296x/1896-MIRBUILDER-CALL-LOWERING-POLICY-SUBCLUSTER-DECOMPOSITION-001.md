# 1896 - MIRBUILDER-CALL-LOWERING-POLICY-SUBCLUSTER-DECOMPOSITION-001

## Token

```text
MIRBUILDER-CALL-LOWERING-POLICY-SUBCLUSTER-DECOMPOSITION-001
```

## Purpose

Decompose the selected `CallLoweringCluster` before selecting any projection
policy.

The priority resolver selects:

```text
projection_policy::UnsupportedDirectShape::shape.call_lowering::FixtureMapped::CallLoweringCluster
```

That cluster mixes diagnostic string helpers, registry predicates, feature
predicates, and call-name canonicalization. Treating the whole cluster as one
projection policy would either hide by-name policy in a large owner or collapse
registry data into ad-hoc Hako code. This card only decomposes the cluster into
machine-checkable subclusters.

## Output

```text
tool:
  tools/rust_lifecycle/
    mirbuilder_call_lowering_policy_subcluster_decomposition.py

fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-call-lowering-policy-subcluster-decomposition-v0.json

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_call_lowering_policy_subcluster_decomposition_guard.sh
```

## Subclusters

```text
DiagnosticStringHelpers:
  generate_self_recursion_warning
  is_commonly_shadowed_method
  suggest_resolution

BuiltinGlobalFunctionRegistry:
  is_builtin_function
  is_math_function

ExternInterfaceRegistry:
  is_env_interface
  is_extern_function

StaticReceiverMethodCatalog:
  has_method

CallFeaturePredicates:
  is_unified_call_enabled
  is_pure_method
  contains_value_return

CallNameCanonicalizationHelpers:
  generate_method_function_name
```

## Decision

```text
kind = SelectSubclusterProjectionPolicy
selected_subcluster = DiagnosticStringHelpers

next_card =
  MIRBUILDER-CALL-LOWERING-DIAGNOSTIC-HELPERS-PROJECTION-POLICY-001
```

Registry-like subclusters require descriptor fixtures before any Hako projection
surface is selected.

## Acceptance

```text
source_count = 12
subcluster_count = 6
all_source_surfaces_classified_exactly_once = 1

whole_cluster_projection_policy = 0
whole_cluster_keep_parent_owner = 0
runtime_or_projection_policy_by_name = 0
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
no whole CallLowering projection policy
no whole CallLowering KeepParentOwner decision
no Hako generation
no HakoAdopted decision
no native source seed
no Source Selfhost claim
```
