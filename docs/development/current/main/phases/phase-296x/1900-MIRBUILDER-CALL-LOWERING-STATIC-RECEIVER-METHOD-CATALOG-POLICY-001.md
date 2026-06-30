# 1900 - MIRBUILDER-CALL-LOWERING-STATIC-RECEIVER-METHOD-CATALOG-POLICY-001

## Token

```text
MIRBUILDER-CALL-LOWERING-STATIC-RECEIVER-METHOD-CATALOG-POLICY-001
```

## Purpose

Resolve the `StaticReceiverMethodCatalog` subcluster selected by the
CallLowering subcluster decomposition.

The selected predicate is:

```text
has_method(box_name, method) -> bool
```

This card materializes a source-extracted static receiver method catalog
descriptor. Explicit method lists remain explicit, while StringBox, ArrayBox,
and MapBox remain delegated to their existing surface catalog resolvers. This
card does not expand those delegated catalogs and does not emit Hako projection
code.

## Output

```text
tool:
  tools/rust_lifecycle/
    mirbuilder_call_lowering_static_receiver_method_catalog_policy.py

fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-call-lowering-static-receiver-method-catalog-policy-v0.json

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_call_lowering_static_receiver_method_catalog_policy_guard.sh
```

## Decision

```text
policy = RegistryDescriptorFixture
owner_edge = mirbuilder::call_lowering_static_receiver_method_catalog
registry_descriptor_selected = 1
projection_surface_selected = 0
delegated_catalogs_expanded = 0

next_card =
  MIRBUILDER-CALL-LOWERING-FEATURE-PREDICATES-PROJECTION-POLICY-001
```

## Evidence

```text
source_count = 1

catalog:
  descriptor_id = call_lowering_static_receiver_method_catalog_v1
  source_extraction = rust_match_arms
  entry_count = 6
  explicit_entry_count = 3
  delegated_catalog_entry_count = 3
  conservative_unknown_box_policy = RejectUnknownBoxes

explicit entries:
  ConsoleStd = print, println, log
  IntegerBox = add, sub, mul, div
  MathBox = sin, cos, abs, min, max

delegated entries:
  StringBox = crate::boxes::basic::StringMethodId::from_name
  ArrayBox = crate::boxes::array::ArrayMethodId::from_name
  MapBox = crate::boxes::MapMethodId::from_name
```

## Acceptance

```text
registry_descriptor_selected = 1
projection_surface_selected = 0
delegated_catalogs_expanded = 0
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
no delegated catalog expansion
no Hako projection surface
no generated Hako source
no HakoAdopted decision
no native seed materialization
no Source Selfhost claim
```
