# 2015 - MIRBUILDER-CRATE-WIDE-NATIVE-OWNER-SEED-CAPABILITY-SURVEY-RERUN-009

## Token

```text
MIRBUILDER-CRATE-WIDE-NATIVE-OWNER-SEED-CAPABILITY-SURVEY-RERUN-009
```

## Purpose

Rerun native-owner seed capability after ID scalar directability unlock.

This resolver consumes the ID scalar directability rerun and checks whether a
single native seed owner edge is machine-derived. It does not select by owner
count or materialize a native seed.

## Result

```text
input_id_scalar_row_count = 31
directable_row_count = 19
owner_edge_repair_required_count = 12
directable_owner_edge_count = 4

directable_owner_edge_counts:
  mirbuilder::context_registry = 5
  mirbuilder::emission_ssa_phi = 6
  mirbuilder::join_i_r_plan = 7
  mirbuilder::join_i_r_route_verify = 1

decision:
  SelectIdScalarDomainSeedCandidateClusterResolution

selected_next_card:
  MIRBUILDER-ID-SCALAR-DOMAIN-SEED-CANDIDATE-CLUSTER-RESOLUTION-001
```

## Output

```text
fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-crate-wide-native-owner-seed-capability-survey-rerun-009-v0.json

tool:
  tools/rust_lifecycle/
    mirbuilder_crate_wide_native_owner_seed_capability_survey_rerun_009.py

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_crate_wide_native_owner_seed_capability_survey_rerun_009_guard.sh
```

## Non-Claims

```text
native_seed_materialization = 0
hako_generation = 0
hako_adopted_decision = 0
source_selfhost_claim = 0
manual_family_selection = 0
manual_owner_selection = 0
cluster_size_as_proof = 0
coverage_percentage_as_proof = 0
route_membership_alone_as_proof = 0
generated_artifact_as_native_edit_authority = 0
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
new_python_semantic_projector = 0
runner_semantic_owner = 0
```
