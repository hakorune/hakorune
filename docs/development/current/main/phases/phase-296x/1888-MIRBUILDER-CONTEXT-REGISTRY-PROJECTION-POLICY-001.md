# 1888 - MIRBUILDER-CONTEXT-REGISTRY-PROJECTION-POLICY-001

## Token

```text
MIRBUILDER-CONTEXT-REGISTRY-PROJECTION-POLICY-001
```

## Purpose

Resolve the selected ContextRegistry projection-policy cluster.

The selected surface is the `CompilationContext::with_plugin_sigs` constructor
helper:

```text
with_plugin_sigs(plugin_method_sigs: HashMap<(String, String), MirType>) -> Self
```

This helper initializes the existing `CompilationContext` parent owner with a
plugin signature registry. It does not open a standalone Hako projection
surface.

## Output

```text
tool:
  tools/rust_lifecycle/
    mirbuilder_context_registry_projection_policy.py

fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-context-registry-projection-policy-v0.json

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_context_registry_projection_policy_guard.sh
```

## Decision

```text
policy = KeepParentOwner
owner_edge = mirbuilder::context_registry
projection_surface_selected = 0

next_card =
  MIRBUILDER-PROJECTION-POLICY-CLUSTER-PRIORITY-RESOLUTION-001
```

## Evidence

```text
source_count = 1
symbol = with_plugin_sigs

markers:
  Create a new CompilationContext with plugin method signatures
  plugin_method_sigs
  HashMap<(String, String), MirType>
  Self::new()
  CompilationContext
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
