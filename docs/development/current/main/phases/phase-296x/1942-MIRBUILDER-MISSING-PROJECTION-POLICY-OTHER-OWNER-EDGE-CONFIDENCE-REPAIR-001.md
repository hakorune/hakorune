# 1942 - MIRBUILDER-MISSING-PROJECTION-POLICY-OTHER-OWNER-EDGE-CONFIDENCE-REPAIR-001

## Token

```text
MIRBUILDER-MISSING-PROJECTION-POLICY-OTHER-OWNER-EDGE-CONFIDENCE-REPAIR-001
```

## Purpose

Repair owner-edge confidence for the `OtherMissingProjectionPolicyCluster`
diagnostic rows.

The previous card partitioned 185 rows into source-derived subclusters, but all
rows still had:

```text
known_owner_edge = ""
owner_edge_confidence = None
```

This card adds a machine-derived overlay that maps each row to a file-scoped
owner edge using only `source_path`.

## Output

```text
fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-missing-projection-policy-other-owner-edge-confidence-repair-v0.json

tool:
  tools/rust_lifecycle/
    mirbuilder_missing_projection_policy_other_owner_edge_confidence_repair.py

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_missing_projection_policy_other_owner_edge_confidence_repair_guard.sh
```

## Repair Policy

```text
source_path starts with src/mir/builder/
  -> hakorune_mir_builder::<file-scoped module path>

source_path starts with src/mir/region/
  -> hakorune_mir_region::<file-scoped module path>

confidence:
  FileScoped
```

This is an owner-edge confidence repair only. It does not infer projection
semantics, route eligibility, native source authority, or HakoAdopted state.

## Acceptance

```text
source_report_consumed = 1
other_owner_cluster_consumed = 1
input_other_owner_cluster_count = 185
repaired_row_count = 185
unrepaired_row_count = 0
distinct_repaired_owner_edge_count = 85
all_other_owner_rows_have_repair_attempt = 1
file_scoped_owner_edge_derived_from_source_path = 1
semantic_projection_inference = 0
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
repaired_row_count = 185
unrepaired_row_count = 0
distinct_repaired_owner_edge_count = 85

decision:
  SelectOtherOwnerClusterRerun

selected_next_card:
  MIRBUILDER-MISSING-PROJECTION-POLICY-OTHER-OWNER-CLUSTER-RERUN-001
```

The next card must rerun the Other owner-cluster partition with this overlay.
It must not treat the file-scoped owner edges as projection proof.

## Stop Conditions

Stop for consultation if the next step requires:

```text
manual owner edge selection
projection semantics inferred from file path
new Hako syntax
runtime fallback
new ABI or backend route
VM/interpreter as semantic owner
Source Selfhost claim
```

## Non-Claims

```text
no Hako generation
no native source seed
no HakoAdopted decision
no Source Selfhost claim
```
