# 1813 - MIRBUILDER-GENERATED-ARTIFACT-TO-NATIVE-OWNER-SEED-POLICY-001

## Token

```text
MIRBUILDER-GENERATED-ARTIFACT-TO-NATIVE-OWNER-SEED-POLICY-001
```

## Purpose

Define the promotion policy that allows a generated Hako artifact to become a
candidate for native Hako source owner seed materialization.

This card does not select a family, does not copy generated Hako into native
source, and does not run a HakoAdopted decision. It only fixes the conditions
that a future resolver must satisfy before opening a seed materialization
card.

## Output

```text
fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-generated-artifact-to-native-owner-seed-policy-v0.json

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_generated_artifact_to_native_owner_seed_policy_guard.sh
```

## Result

```text
decision = PolicyDefined
next_resolution =
  MIRBUILDER-GENERATED-ARTIFACT-NATIVE-OWNER-SEED-CANDIDATE-RESOLUTION-001
current_blocker =
  SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001
```

## Acceptance

```text
generated_artifact_is_not_edit_authority = 1
composition_owner_as_semantic_owner = 0
seed_candidate_requires_leaf_semantic_owner = 1
seed_candidate_requires_verified_artifact = 1
seed_candidate_requires_deterministic_regeneration = 1
seed_candidate_requires_oracle_or_contract_green = 1
seed_candidate_requires_no_runtime_fallback = 1
seed_candidate_requires_no_new_abi_or_backend = 1
seed_materialization_is_separate_card = 1
hako_adoption_decision_is_separate_card = 1
manual_family_selection = 0
source_selfhost_claim = 0
```

## Non-Claims

```text
no native source owner materialization
no HakoAdopted decision
no Source Selfhost claim
no generated artifact edit authority
```
