# 1946 - MIRBUILDER-OTHER-UNIT-OBSERVER-SURFACE-PROJECTION-POLICY-001

## Token

```text
MIRBUILDER-OTHER-UNIT-OBSERVER-SURFACE-PROJECTION-POLICY-001
```

## Purpose

Materialize the projection-policy descriptor for
`shape.other_unit_observer_surface`.

The selected surfaces are unit-returning observer / annotation helpers with:

```text
return_family = unit
borrow_axis = NoBorrow
type_transport_axis = Known
owner_edge_confidence = FileScoped
```

This card records the descriptor only. It does not generate Hako and does not
claim native source authority.

## Output

```text
fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-other-unit-observer-surface-projection-policy-v0.json

tool:
  tools/rust_lifecycle/
    mirbuilder_other_unit_observer_surface_projection_policy.py

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_other_unit_observer_surface_projection_policy_guard.sh
```

## Acceptance

```text
shape_signature_cluster_resolution_consumed = 1
shape_signature_inventory_consumed = 1
unconverted_surface_report_consumed = 1
selected_shape_signature = shape.other_unit_observer_surface
source_count = 26
descriptor_selected = 1
hako_projection_selected = 0
return_contract = unit
mutation_frame = []
returned_borrow = 0
receiver_borrow = 0
manual_family_selection = 0
cluster_size_as_proof = 0
coverage_percentage_as_proof = 0
route_membership_alone_as_proof = 0
generated_artifact_as_edit_authority = 0
hako_generation = 0
hako_adopted_decision = 0
native_source_seed_materialization = 0
source_selfhost_claim = 0
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
new_python_semantic_projector = 0
runner_semantic_owner = 0
```

## Result

```text
descriptor:
  other_unit_observer_surface_v1

source_count:
  26

decision:
  SelectProjectionPolicyDescriptor

selected_next_card:
  MIRBUILDER-PROJECTION-POLICY-CLUSTER-PRIORITY-RESOLUTION-001
```

## Recommended Next Task

```text
MIRBUILDER-PROJECTION-POLICY-CLUSTER-PRIORITY-RESOLUTION-001
```

Rerun the cluster-priority resolver so the completed descriptor can be excluded
and the next unclosed projection-policy cluster can be selected.

## Non-Claims

```text
no Hako projection
no native source seed
no HakoAdopted decision
no Source Selfhost claim
```
