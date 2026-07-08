# 2116 - MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-PUSH-SURFACE-HAKO-PARITY-PILOT-001

## Token

```text
MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-PUSH-SURFACE-HAKO-PARITY-PILOT-001
```

## Purpose

Land the first narrow Write `.hako` implementation pilot for
`PushSurfacePolicy / ArrayAppendAny`.

This card consumes the 2115 Rust oracle fixture and adds the hand-authored
classifier/policy mirror. It is an implementation pilot only: the parity gate,
direct closeout decision, and Hako adoption decision remain separate cards.

## Implementation

```text
hako_source:
  lang/src/compiler/lib/write_push_surface_policy_classifier.hako

box:
  WritePushSurfacePolicyClassifierBox

method:
  classify(route_kind)
```

## Included Surface

```text
input_route_kind = ArrayAppendAny
case_id = array_append_any_push_surface
subsurface_id = PushSurfacePolicy
core_method_op = ArrayPush
core_method_lowering_tier = ColdFallback
result_class = ScalarI64Result
return_shape = ScalarI64
value_demand = WriteAny
publication_policy = NoPublication
effect_class = mutate
mutation_class = MutatesReceiverOrContainer
hako_role = classifier_policy_mirror_only
```

## Result

```text
write_push_surface_hako_parity_pilot = 1
hako_implementation_landed = 1
hako_source_verifies = 1
array_append_any_scope = 1
push_surface_policy_scope = 1
classifier_policy_mirror_only = 1
parity_gate_required = 1

runtime_mutation_authority = 0
write_direct_closeout_materialized = 0
write_result_policy_ready = 0
write_scalar_i64_routes_closeout = 0
scalar_known_transport_axis_closeout = 0
hako_adoption = 0
source_selfhost_claim = 0
```

## Decision

```text
decision:
  SelectWritePushSurfaceParityGate

reason_token:
  WritePushSurfaceHakoParityPilotLanded

selected_next_card:
  MIRBUILDER-CARRIER-TYPE-SCALAR-KNOWN-WRITE-PUSH-SURFACE-PARITY-GATE-001
```

## Guard

```text
tools/checks/
  rust_lifecycle_mirbuilder_carrier_type_scalar_known_write_push_surface_hako_parity_pilot_guard.sh
```

## Output

```text
fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-carrier-type-scalar-known-write-push-surface-hako-parity-pilot-v0.json

tool:
  tools/rust_lifecycle/
    mirbuilder_carrier_type_scalar_known_write_push_surface_hako_parity_pilot.py

hako:
  lang/src/compiler/lib/write_push_surface_policy_classifier.hako
```

## Non-Claims

```text
write_subsurface_selected = 0
write_direct_closeout_materialized = 0
write_result_policy_ready = 0
write_scalar_i64_routes_closeout = 0
scalar_known_transport_axis_closeout = 0
component_specific_direct_contract_materialized = 0
hako_adoption = 0
source_selfhost_claim = 0
new_route_authority = 0
behavior_change = 0
runtime_mutation_authority = 0
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
native_seed_materialization = 0
hako_generation = 0
hako_adopted_decision = 0
manual_subsurface_selection = 0
route_count_as_proof = 0
apparent_simplicity_as_proof = 0
accepted_read_contract_similarity_as_proof = 0
```
