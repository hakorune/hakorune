# 1905 - MIRBUILDER-CALL-LOWERING-NAME-CANONICALIZATION-PROJECTION-POLICY-001

## Token

```text
MIRBUILDER-CALL-LOWERING-NAME-CANONICALIZATION-PROJECTION-POLICY-001
```

## Purpose

Resolve the `CallNameCanonicalizationHelpers` subcluster from the CallLowering
policy decomposition.

The selected source surface is:

```text
generate_method_function_name(box_name, method_name, arity) -> String
```

This surface is a deterministic naming helper. This card materializes the
source-extracted canonical name descriptor and returns to the projection-policy
cluster resolver. It does not open a standalone Hako projection surface.

## Output

```text
tool:
  tools/rust_lifecycle/
    mirbuilder_call_lowering_name_canonicalization_projection_policy.py

fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-call-lowering-name-canonicalization-projection-policy-v0.json

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_call_lowering_name_canonicalization_projection_policy_guard.sh
```

## Decision

```text
policy = MaterializeNameCanonicalizationDescriptor
descriptor_kind = MethodFunctionNameCanonicalizationV1
source_extracted_descriptor = 1
projection_surface_selected = 0

next_card =
  MIRBUILDER-PROJECTION-POLICY-CLUSTER-PRIORITY-RESOLUTION-001
```

## Evidence

```text
source_count = 1
source_surface = generate_method_function_name
format = "{}.{}/{}"
parts = box_name "." method_name "/" arity
```

## Acceptance

```text
source_extracted_descriptor = 1
format = "{}.{}/{}"
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
