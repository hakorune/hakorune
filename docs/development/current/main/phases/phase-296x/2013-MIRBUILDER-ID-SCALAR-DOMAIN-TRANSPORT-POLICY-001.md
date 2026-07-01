# 2013 - MIRBUILDER-ID-SCALAR-DOMAIN-TRANSPORT-POLICY-001

## Token

```text
MIRBUILDER-ID-SCALAR-DOMAIN-TRANSPORT-POLICY-001
```

## Purpose

Define the nominal scalar transport policy for MirBuilder ID domain return
types selected by `MIRBUILDER-DOMAIN-OBJECT-ID-TRANSPORT-POLICY-INVENTORY-001`.

This card selects transport vocabulary only. It does not make ID types raw
interchangeable `i64`, does not choose object layout transport, and does not
emit Hako.

## Policy

```text
policy_id = NominalIdScalarDomainTransportV1
physical_lane = i64

nominal_transports:
  ValueId       -> ValueIdAsI64
  BasicBlockId  -> BasicBlockIdAsI64
  BindingId     -> BindingIdAsI64
  BodyId        -> BodyIdAsI64
  SlotId        -> SlotIdAsI64
  TypedValueId  -> TypedValueIdAsI64
```

The physical lane is shared, but the semantic transport remains nominal.

## Result

```text
id_scalar_input_count = 31

canonical_id_type_counts:
  ValueId = 17
  BasicBlockId = 9
  BindingId = 2
  BodyId = 1
  SlotId = 1
  TypedValueId = 1

owner_edge_confidence_counts:
  FixtureMapped = 19
  None = 12

decision:
  SelectIdScalarDomainTransportDirectabilityRerun

selected_next_card:
  MIRBUILDER-ID-SCALAR-DOMAIN-TRANSPORT-DIRECTABILITY-RERUN-001
```

## Output

```text
fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-id-scalar-domain-transport-policy-v0.json

tool:
  tools/rust_lifecycle/
    mirbuilder_id_scalar_domain_transport_policy.py

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_id_scalar_domain_transport_policy_guard.sh
```

## Non-Claims

```text
raw_i64_interchangeability = 0
object_layout_transport = 0
generator_object_transport = 0
invalid_sentinel_semantics = 0
reserved_id_policy = 0
manual_family_selection = 0
manual_shape_selection = 0
manual_axis_selection = 0
manual_carrier_selection = 0
domain_object_count_as_proof = 0
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
