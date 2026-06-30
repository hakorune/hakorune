# 1920 - MIRBUILDER-BODY-VIEW-FEATURE-PROJECTION-POLICY-001

## Token

```text
MIRBUILDER-BODY-VIEW-FEATURE-PROJECTION-POLICY-001
```

## Purpose

Resolve the selected `BodyViewFeatureCluster` from the projection-policy
priority resolver.

This card materializes a source-extracted read-only view length descriptor for
`BodyView::len`. It does not select Hako projection or generate Hako.

## Output

```text
tool:
  tools/rust_lifecycle/
    mirbuilder_body_view_feature_projection_policy.py

fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-body-view-feature-projection-policy-v0.json

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_body_view_feature_projection_policy_guard.sh
```

## Descriptor

```text
BodyView::len:
  Recipe -> RecipeBody::len
  Slice -> slice::len
  return_contract = usize
  mutation_frame = []
  returned_borrow = 0
```

## Decision

```text
kind = SelectProjectionPolicyDescriptor

next_card =
  MIRBUILDER-PROJECTION-POLICY-CLUSTER-PRIORITY-RESOLUTION-001
```

## Acceptance

```text
source_count = 1
descriptor_id = body_view_feature_len_v1
descriptor_selected = 1
hako_projection_selected = 0
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
no Hako projection selected
no Hako generation
no HakoAdopted decision
no native seed materialization
no Source Selfhost claim
```
