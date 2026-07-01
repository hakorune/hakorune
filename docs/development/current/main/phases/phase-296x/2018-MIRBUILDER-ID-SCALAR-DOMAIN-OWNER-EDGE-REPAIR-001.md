# 2018 - MIRBUILDER-ID-SCALAR-DOMAIN-OWNER-EDGE-REPAIR-001

## Token

```text
MIRBUILDER-ID-SCALAR-DOMAIN-OWNER-EDGE-REPAIR-001
```

## Purpose

Repair the 12 ID scalar directability rows that were blocked only by missing
owner-edge confidence.

This card does not choose a native seed owner. It consumes the existing Other
owner-edge confidence repair fixture by exact `source_id` match and produces a
complete repaired owner-edge map for the ID scalar rows.

## Result

```text
input_repair_required_count = 12
repaired_row_count = 12
unrepaired_row_count = 0
distinct_repaired_owner_edge_count = 6

decision:
  SelectSeedReadinessResolutionRerun

reason_token:
  IdScalarOwnerEdgeRepairComplete

selected_next_card:
  MIRBUILDER-ID-SCALAR-DOMAIN-SEED-READINESS-RESOLUTION-002
```

## Repaired Owner Edges

```text
hakorune_mir_builder::builder_init = 1
hakorune_mir_builder::builder_value_kind = 1
hakorune_mir_builder::joinir_id_remapper = 1
hakorune_mir_builder::utils::id_alloc = 3
hakorune_mir_builder::utils::local_ssa = 5
hakorune_mir_region::function_slot_registry = 1
```

## Output

```text
fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-id-scalar-domain-owner-edge-repair-v0.json

tool:
  tools/rust_lifecycle/
    mirbuilder_id_scalar_domain_owner_edge_repair.py

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_id_scalar_domain_owner_edge_repair_guard.sh
```

## Non-Claims

```text
manual_owner_selection = 0
family_name_based_policy = 0
cluster_size_as_proof = 0
coverage_percentage_as_proof = 0
route_membership_alone_as_proof = 0
raw_i64_interchangeability = 0
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
