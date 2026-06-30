# 1944 - MIRBUILDER-MISSING-PROJECTION-POLICY-OTHER-SHAPE-SIGNATURE-INVENTORY-001

## Token

```text
MIRBUILDER-MISSING-PROJECTION-POLICY-OTHER-SHAPE-SIGNATURE-INVENTORY-001
```

## Purpose

Assign shape-signature candidates to the repaired `OtherMissingProjectionPolicy`
subclusters.

The previous rerun proved that all 185 rows now have `FileScoped` owner-edge
confidence, but all rows still had `shape_signature = unknown_shape`. This card
uses generic return / borrow / type transport axes to assign diagnostic
`shape.other_*` candidates.

## Output

```text
fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-missing-projection-policy-other-shape-signature-inventory-v0.json

tool:
  tools/rust_lifecycle/
    mirbuilder_missing_projection_policy_other_shape_signature_inventory.py

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_missing_projection_policy_other_shape_signature_inventory_guard.sh
```

## Shape Assignment Policy

```text
input axes:
  return_family
  borrow_axis
  type_transport_axis

output:
  shape.other_* diagnostic candidate
```

This inventory does not infer projection semantics and does not select a
projection policy. It only removes the `unknown_shape` blocker for the repaired
Other owner rows.

## Acceptance

```text
other_owner_cluster_rerun_consumed = 1
input_other_owner_cluster_count = 185
input_subcluster_count = 123
assigned_subcluster_count = 123
assigned_row_count = 185
shape_signature_count = 11
unknown_shape_count_after_inventory = 0
semantic_projection_inference = 0
family_name_based_policy = 0
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
shape_signature_count = 11
unknown_shape_count_after_inventory = 0

largest diagnostic shapes:
  shape.other_mutating_result_surface = 42 rows
  shape.other_optional_read_surface = 37 rows
  shape.other_unit_observer_surface = 26 rows
  shape.other_custom_carrier_surface = 21 rows

decision:
  SelectOtherShapeSignatureClusterResolution

selected_next_card:
  MIRBUILDER-MISSING-PROJECTION-POLICY-OTHER-SHAPE-SIGNATURE-CLUSTER-RESOLUTION-001
```

## Stop Conditions

Stop for consultation if the next step requires:

```text
manual shape selection
shape policy inferred from family names
new Hako syntax
runtime fallback
new ABI or backend route
VM/interpreter as semantic owner
Source Selfhost claim
```

## Non-Claims

```text
no projection policy selected
no Hako generation
no native source seed
no HakoAdopted decision
no Source Selfhost claim
```
