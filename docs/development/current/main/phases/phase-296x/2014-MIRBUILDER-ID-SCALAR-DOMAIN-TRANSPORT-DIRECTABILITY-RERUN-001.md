# 2014 - MIRBUILDER-ID-SCALAR-DOMAIN-TRANSPORT-DIRECTABILITY-RERUN-001

## Token

```text
MIRBUILDER-ID-SCALAR-DOMAIN-TRANSPORT-DIRECTABILITY-RERUN-001
```

## Purpose

Rerun directability for the 31 ID scalar rows after
`NominalIdScalarDomainTransportV1` is defined.

This card does not materialize native seeds. It separates rows that are ready
for native-owner seed capability survey from rows that still need owner-edge
repair.

## Result

```text
input_id_scalar_row_count = 31
directable_with_nominal_id_scalar_transport_count = 19
owner_edge_repair_required_count = 12

owner_edge_counts:
  mirbuilder::context_registry = 5
  mirbuilder::emission_ssa_phi = 6
  mirbuilder::join_i_r_plan = 7
  mirbuilder::join_i_r_route_verify = 1
  <none> = 12

decision:
  SelectNativeOwnerSeedCapabilitySurveyRerun

selected_next_card:
  MIRBUILDER-CRATE-WIDE-NATIVE-OWNER-SEED-CAPABILITY-SURVEY-RERUN-009
```

## Output

```text
fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-id-scalar-domain-transport-directability-rerun-v0.json

tool:
  tools/rust_lifecycle/
    mirbuilder_id_scalar_domain_transport_directability_rerun.py

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_id_scalar_domain_transport_directability_rerun_guard.sh
```

## Non-Claims

```text
manual_family_selection = 0
manual_shape_selection = 0
manual_axis_selection = 0
manual_owner_selection = 0
raw_i64_interchangeability = 0
object_layout_transport = 0
generated_artifact_as_native_edit_authority = 0
native_seed_materialization = 0
hako_generation = 0
hako_adopted_decision = 0
source_selfhost_claim = 0
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
new_python_semantic_projector = 0
runner_semantic_owner = 0
```
