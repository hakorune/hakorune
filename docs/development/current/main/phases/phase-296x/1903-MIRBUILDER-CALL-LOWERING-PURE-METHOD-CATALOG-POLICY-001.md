# 1903 - MIRBUILDER-CALL-LOWERING-PURE-METHOD-CATALOG-POLICY-001

## Token

```text
MIRBUILDER-CALL-LOWERING-PURE-METHOD-CATALOG-POLICY-001
```

## Purpose

Resolve the `PureMethodCatalog` subcluster selected after the unified-call mode
gate.

The selected source surface is:

```text
EffectsAnalyzerBox::is_pure_method(box_name, method) -> bool
```

This surface is a source catalog predicate over `match (box_name, method)`. This
card materializes that catalog as a descriptor fixture. It does not select a
standalone Hako projection surface and does not make the catalog a native
semantic owner.

## Output

```text
tool:
  tools/rust_lifecycle/
    mirbuilder_call_lowering_pure_method_catalog_policy.py

fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-call-lowering-pure-method-catalog-policy-v0.json

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_call_lowering_pure_method_catalog_policy_guard.sh
```

## Decision

```text
policy = MaterializeCatalogDescriptor
descriptor_kind = PureMethodCatalogDescriptorV1
source_extracted_catalog = 1
projection_surface_selected = 0

next_card =
  MIRBUILDER-CALL-LOWERING-VALUE-RETURN-AST-SCAN-PROJECTION-POLICY-001
```

## Evidence

```text
source_count = 1
source_surface = is_pure_method
catalog_source = match (box_name, method)
box_count = 4
entry_count = 10
```

## Acceptance

```text
source_extracted_catalog = 1
box_count = 4
entry_count = 10
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
