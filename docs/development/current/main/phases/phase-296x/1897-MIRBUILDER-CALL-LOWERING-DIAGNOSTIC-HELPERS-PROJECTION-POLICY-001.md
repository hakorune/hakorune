# 1897 - MIRBUILDER-CALL-LOWERING-DIAGNOSTIC-HELPERS-PROJECTION-POLICY-001

## Token

```text
MIRBUILDER-CALL-LOWERING-DIAGNOSTIC-HELPERS-PROJECTION-POLICY-001
```

## Purpose

Resolve the `DiagnosticStringHelpers` subcluster selected by the CallLowering
subcluster decomposition.

The selected helpers are diagnostic-only message and warning helpers:

```text
generate_self_recursion_warning(box_name, method) -> String
is_commonly_shadowed_method(name) -> bool
suggest_resolution(name) -> String
```

They do not own call routing, registry selection, runtime fallback, or Hako
projection policy. They remain parent-owned under CallLowering diagnostics.

## Output

```text
tool:
  tools/rust_lifecycle/
    mirbuilder_call_lowering_diagnostic_helpers_projection_policy.py

fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-call-lowering-diagnostic-helpers-projection-policy-v0.json

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_call_lowering_diagnostic_helpers_projection_policy_guard.sh
```

## Decision

```text
policy = KeepParentOwner
owner_edge = mirbuilder::call_lowering_diagnostic_helpers
projection_surface_selected = 0
registry_descriptor_selected = 0

next_card =
  MIRBUILDER-CALL-LOWERING-BUILTIN-GLOBAL-FUNCTION-REGISTRY-POLICY-001
```

## Evidence

```text
source_count = 3

roles:
  self_recursion_warning_message = 1
  diagnostic_shadow_warning_predicate = 1
  unresolved_function_hint_message = 1

markers:
  Check if method is commonly shadowed (for warning generation)
  Generate warning about potential self-recursion
  Suggest resolution for unresolved function
  Did you mean 'env.console.log' or 'print'?
  Check function name or ensure it's in scope.
```

## Acceptance

```text
policy = KeepParentOwner
projection_surface_selected = 0
registry_descriptor_selected = 0
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
no standalone Hako projection surface
no registry descriptor fixture
no Hako generation
no HakoAdopted decision
no native seed materialization
no Source Selfhost claim
```
