# 1898 - MIRBUILDER-CALL-LOWERING-BUILTIN-GLOBAL-FUNCTION-REGISTRY-POLICY-001

## Token

```text
MIRBUILDER-CALL-LOWERING-BUILTIN-GLOBAL-FUNCTION-REGISTRY-POLICY-001
```

## Purpose

Resolve the `BuiltinGlobalFunctionRegistry` subcluster selected by the
CallLowering subcluster decomposition.

The selected predicates are registry membership checks:

```text
is_builtin_function(name) -> bool
is_math_function(name) -> bool
```

This card does not copy the name lists into Hako code. It materializes a
source-extracted registry descriptor fixture so later projection work can
consume descriptor data rather than ad-hoc by-name branches.

## Output

```text
tool:
  tools/rust_lifecycle/
    mirbuilder_call_lowering_builtin_global_function_registry_policy.py

fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-call-lowering-builtin-global-function-registry-policy-v0.json

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_call_lowering_builtin_global_function_registry_policy_guard.sh
```

## Decision

```text
policy = RegistryDescriptorFixture
owner_edge = mirbuilder::call_lowering_builtin_global_function_registry
registry_descriptor_selected = 1
projection_surface_selected = 0

next_card =
  MIRBUILDER-CALL-LOWERING-EXTERN-INTERFACE-REGISTRY-POLICY-001
```

## Evidence

```text
source_count = 2

registry:
  descriptor_id = call_lowering_builtin_global_function_registry_v1
  source_extraction = rust_matches_string_literals
  builtin_function_count = 12
  math_function_count = 9
  shared_builtin_math_count = 5

entries:
  abs
  ceil
  cos
  error
  exit
  floor
  gc_collect
  gc_stats
  max
  min
  now
  panic
  pow
  print
  sin
  sqrt
```

## Acceptance

```text
registry_descriptor_selected = 1
projection_surface_selected = 0
ad_hoc_by_name_policy = 0
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
no Hako projection surface
no generated Hako source
no HakoAdopted decision
no native seed materialization
no Source Selfhost claim
```
