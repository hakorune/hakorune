# 1993 - MIRBUILDER-DIRECT-STATE-PLAN-REFRESH-HAKO-ADOPTION-DECISION-001

## Token

```text
MIRBUILDER-DIRECT-STATE-PLAN-REFRESH-HAKO-ADOPTION-DECISION-001
```

## Purpose

Decide the HakoAdopted state for the `direct_state_plan_refresh` native source
seed.

This card adopts the narrow `direct_state_plan_refresh` native source owner. It
does not claim Source Selfhost, delete Rust, add a runtime fallback, or add a
new backend / ABI / canonical MIR instruction.

## Output

```text
fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-direct-state-plan-refresh-hako-adoption-decision-v0.json

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_direct_state_plan_refresh_hako_adoption_decision_guard.sh
```

## Acceptance

```text
native_source_seed_present = 1
BridgePolicyV2 consumed = 1
derived_artifact_verifier = VerifiedHakoFamilyIR
direct_state_plan_refresh = 1
direct_state_plan_refresh_only = 1
canonical_json_parity = 1
decision = Adopt
selected_next_route = native_hako_source_owner

hako_adopted = 1
native_hako_source_owner_present = 1
rust_bootstrap_retained = 1
rust_oracle_retained = 1

manual_family_selection = 0
source_selfhost_claim = 0
rust_deletion = 0
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
new_canonical_mir_instruction = 0
new_python_semantic_projector = 0
runner_semantic_owner = 0
generated_artifact_as_edit_authority = 0
cluster_size_as_proof = 0
coverage_percentage_as_proof = 0
route_membership_alone_as_proof = 0
```

## Result

```text
decision:
  Adopt

reason_token:
  DirectStatePlanRefreshNativeSeedPresentAndBridgePolicyV2Green

selected_next_card:
  MIRBUILDER-STRICT-CONVERTER-EMISSION-NATIVE-SEED-CANDIDATE-SELECTION-RERUN-003
```

## Non-Claims

```text
no Source Selfhost claim
no Rust deletion
no runtime fallback
no new backend route
no new ABI
no new canonical MIR instruction
no new Python SemanticProjector
no runner semantic ownership
```
