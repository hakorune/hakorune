# 1899 - MIRBUILDER-CALL-LOWERING-EXTERN-INTERFACE-REGISTRY-POLICY-001

## Token

```text
MIRBUILDER-CALL-LOWERING-EXTERN-INTERFACE-REGISTRY-POLICY-001
```

## Purpose

Resolve the `ExternInterfaceRegistry` subcluster selected by the CallLowering
subcluster decomposition.

The selected predicates are extern/interface membership checks:

```text
is_extern_function(name) -> bool
is_env_interface(name) -> bool
```

This card materializes a source-extracted registry descriptor for extern
prefixes and supported env interfaces. It does not select the wider env method
spec table and does not emit Hako projection code.

## Output

```text
tool:
  tools/rust_lifecycle/
    mirbuilder_call_lowering_extern_interface_registry_policy.py

fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-call-lowering-extern-interface-registry-policy-v0.json

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_call_lowering_extern_interface_registry_policy_guard.sh
```

## Decision

```text
policy = RegistryDescriptorFixture
owner_edge = mirbuilder::call_lowering_extern_interface_registry
registry_descriptor_selected = 1
projection_surface_selected = 0
method_spec_surface_selected = 0

next_card =
  MIRBUILDER-CALL-LOWERING-STATIC-RECEIVER-METHOD-CATALOG-POLICY-001
```

## Evidence

```text
source_count = 2

extern_prefixes:
  nyash.
  env.
  system.

env_interfaces:
  env
  env.canvas
  env.codegen
  env.console
  env.fs
  env.future
  env.net
  env.process
  env.task
```

## Acceptance

```text
registry_descriptor_selected = 1
projection_surface_selected = 0
method_spec_surface_selected = 0
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
no env method spec projection
no Hako projection surface
no generated Hako source
no HakoAdopted decision
no native seed materialization
no Source Selfhost claim
```
