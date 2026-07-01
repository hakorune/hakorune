# 2020 - MIRBUILDER-ID-SCALAR-SEED-EVIDENCE-CONTRACT-001

## Token

```text
MIRBUILDER-ID-SCALAR-SEED-EVIDENCE-CONTRACT-001
```

## Purpose

Define the evidence contract that lets ID scalar directability feed seed packet
generation without treating directability alone as native seed evidence.

This card does not select an owner edge, materialize a native seed, generate
Hako, or claim Source Selfhost.

## Result

```text
contract_id = IdScalarSeedEvidencePacketV1
directability_only_is_seed_evidence = 0
directability_may_feed_seed_packet_generation = 1

required_packet_components:
  SourcePlanAndRecipe
  VerifierResultFixture
  DerivedArtifactSeedDraftInput

decision:
  PolicyDefined

selected_next_card:
  MIRBUILDER-ID-SCALAR-SEED-PACKET-CANDIDATE-SELECTION-001
```

## Output

```text
fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-id-scalar-seed-evidence-contract-v0.json

tool:
  tools/rust_lifecycle/
    mirbuilder_id_scalar_seed_evidence_contract.py

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_id_scalar_seed_evidence_contract_guard.sh
```

## Non-Claims

```text
manual_owner_selection = 0
cluster_size_as_proof = 0
directable_row_count_as_proof = 0
lexical_tiebreaker_as_seed_selection_proof = 0
coverage_percentage_as_proof = 0
route_membership_alone_as_proof = 0
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
