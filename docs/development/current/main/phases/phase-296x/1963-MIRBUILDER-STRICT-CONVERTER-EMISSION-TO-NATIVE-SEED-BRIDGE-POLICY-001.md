# 1963 - MIRBUILDER-STRICT-CONVERTER-EMISSION-TO-NATIVE-SEED-BRIDGE-POLICY-001

## Token

```text
MIRBUILDER-STRICT-CONVERTER-EMISSION-TO-NATIVE-SEED-BRIDGE-POLICY-001
```

## Purpose

Define the bridge policy that allows strict converter emission evidence to feed
native source seed candidate selection without treating generated artifacts as
native edit authority.

This card only defines the bridge. It does not generate Hako, materialize a
native source seed, make a HakoAdopted decision, or claim Source Selfhost.

## Output

```text
fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-strict-converter-emission-to-native-seed-bridge-policy-v0.json

tool:
  tools/rust_lifecycle/
    mirbuilder_strict_converter_emission_to_native_seed_bridge_policy.py

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_strict_converter_emission_to_native_seed_bridge_policy_guard.sh
```

## Acceptance

```text
native_seed_survey_rerun_003_consumed = 1
previous_decision = KeepStopped
previous_reason =
  NoMachineDerivedNativeOwnerSeedCandidateAfterStrictEmissionProbe

strict_converter_emission_probe_consumed = 1
strict_verified_hako_family_ir_count = 47

generated_artifact_as_native_edit_authority = 0
generated_artifact_as_seed_draft_input = 1
seed_draft_input_state_name = DerivedArtifactSeedDraftInput
native_seed_state_name = NativeSourceSeed
hako_adopted_state_name = HakoAdopted

candidate_requires_verified_hako_family_ir = 1
candidate_requires_deterministic_regeneration = 1
candidate_requires_owner_edge_confidence_exact_or_fixture = 1
candidate_requires_provenance_manifest = 1
candidate_requires_verifier_or_oracle_or_guard = 1
candidate_requires_no_borrow_gap = 1
candidate_requires_no_carrier_gap = 1
candidate_requires_no_type_transport_gap = 1

native_seed_materialization = 0
hako_generation = 0
hako_adopted_decision = 0
source_selfhost_claim = 0
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
new_python_semantic_projector = 0
runner_semantic_owner = 0
manual_family_selection = 0
cluster_size_as_proof = 0
coverage_percentage_as_proof = 0
route_membership_alone_as_proof = 0
```

## Result

```text
decision:
  PolicyDefined

reason_token:
  StrictEmissionToNativeSeedBridgePolicyDefined

selected_next_card:
  MIRBUILDER-STRICT-CONVERTER-EMISSION-NATIVE-SEED-CANDIDATE-SELECTION-001
```

## Non-Claims

```text
generated artifact is not native edit authority
no Hako generation
no native source seed materialization
no HakoAdopted decision
no Source Selfhost claim
no runtime fallback
no new backend route
no new ABI
no new Python SemanticProjector
no runner semantic ownership
```
