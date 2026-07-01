# 2006 - MIRBUILDER-CRATE-WIDE-NATIVE-OWNER-SEED-CAPABILITY-SURVEY-RERUN-008

## Token

```text
MIRBUILDER-CRATE-WIDE-NATIVE-OWNER-SEED-CAPABILITY-SURVEY-RERUN-008
```

## Purpose

Rerun native-owner seed capability after projection descriptor coverage
reclassification moved landed descriptor-covered rows out of
`MissingProjectionPolicy`.

This resolver checks whether the fresh source-surface report exposes a new
strict-emission bridge-eligible native seed candidate. It does not materialize
a native seed, run Hako generation, run an adoption decision, or claim Source
Selfhost.

## Result

```text
projection_descriptor_coverage_reclassified_count = 380
missing_projection_policy_count = 1004
mapped_to_known_owner_count = 398

verified_hako_family_ir_count = 47
bridge_eligible_count = 0
already_adopted_count = 15

decision:
  SelectNativeOwnerCheckpointRerun

selected_next_card:
  SOURCE-SELFHOST-NATIVE-OWNER-CHECKPOINT-RERUN-001
```

The report changed the blocker surface, but strict-emission bridge selection
still has no seed candidate. The next machine-derived step is to rerun the
native-owner checkpoint against the fresh report instead of choosing a blocker
lane by hand.

## Output

```text
fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-crate-wide-native-owner-seed-capability-survey-rerun-008-v0.json

tool:
  tools/rust_lifecycle/
    mirbuilder_crate_wide_native_owner_seed_capability_survey_rerun_008.py

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_crate_wide_native_owner_seed_capability_survey_rerun_008_guard.sh
```

## Non-Claims

```text
manual_family_selection = 0
cluster_size_as_proof = 0
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
